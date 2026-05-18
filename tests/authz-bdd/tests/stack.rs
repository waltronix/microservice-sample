//! Stack bootstrap — starts the full compose stack once per test binary run.
//!
//! Which backend is used is controlled by the `AUTHZ_BACKEND` environment
//! variable (default: `openfga`):
//!
//! ```
//! # OpenFGA (default)
//! cargo test -p authz-bdd
//!
//! # OPA  — requires OPA-built service images (see README)
//! AUTHZ_BACKEND=opa cargo test -p authz-bdd
//! ```
//!
//! Boot sequence:
//!   1. Bring up the backend infra (OpenFGA or OPA) and wait for readiness.
//!   2. Provision: write the model/policy, write an env file consumed by the services.
//!   3. Bring up the three services against that backend.

use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

use serde_json::Value;

// ---------------------------------------------------------------------------
// AuthzSetup — backend-specific state needed by bootstrap steps
// ---------------------------------------------------------------------------

pub enum AuthzSetup {
    OpenFga {
        url: String,
        store_id: String,
        model_id: String,
    },
    Opa {
        url: String,
    },
}

pub struct Stack {
    pub compose_file: String,
    pub authz: AuthzSetup,
    pub user_url: String,
    pub book_url: String,
    pub review_url: String,
}

static STACK: OnceLock<Stack> = OnceLock::new();

fn compose_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn model_file() -> std::path::PathBuf {
    compose_dir().join("../../libraries/authz/fga/model.fga")
}

fn rego_policy_file() -> std::path::PathBuf {
    compose_dir().join("../../libraries/authz/opa/library.rego")
}

fn authz_env_file() -> std::path::PathBuf {
    compose_dir().join("openfga.env")
}

pub fn run(program: &str, args: &[&str]) -> std::process::Output {
    let out = Command::new(program)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run `{program}`: {e}"));
    assert!(
        out.status.success(),
        "`{program} {}` failed:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&out.stderr)
    );
    out
}

fn compose_up(file: &str, services: &[&str]) {
    let mut args = vec!["-f", file, "up", "-d", "--wait"];
    args.extend_from_slice(services);
    run("docker-compose", &args);
}

fn compose_port(file: &str, service: &str, port: u16) -> u16 {
    let port_str = port.to_string();
    let out = run("docker-compose", &["-f", file, "port", service, &port_str]);
    let s = String::from_utf8_lossy(&out.stdout);
    s.trim()
        .rsplit(':')
        .next()
        .expect("port number in docker-compose port output")
        .trim()
        .parse()
        .expect("parse port as u16")
}

// ---------------------------------------------------------------------------
// OpenFGA provisioning
// ---------------------------------------------------------------------------

fn provision_openfga(openfga_url: &str) -> (String, String) {
    let model_path = model_file();
    let model_str = model_path.to_str().expect("model path");

    let out = run("fga", &["store", "create", "--api-url", openfga_url, "--name", "bdd-e2e"]);
    let parsed: Value = serde_json::from_slice(&out.stdout).expect("parse store create");
    let store_id = parsed["store"]["id"].as_str().expect("store id").to_owned();

    let out = run("fga", &[
        "model", "write",
        "--api-url", openfga_url,
        "--store-id", &store_id,
        "--file", model_str,
    ]);
    let parsed: Value = serde_json::from_slice(&out.stdout).expect("parse model write");
    let model_id = parsed["authorization_model_id"].as_str().expect("model id").to_owned();

    (store_id, model_id)
}

/// Write a tuple directly via `fga`, bypassing the service layer.
/// Used to bootstrap system admins where no REST endpoint exists.
pub fn fga_write_tuple(
    openfga_url: &str,
    store_id: &str,
    model_id: &str,
    user: &str,
    relation: &str,
    object: &str,
) {
    run("fga", &[
        "tuple", "write",
        "--api-url", openfga_url,
        "--store-id", store_id,
        "--model-id", model_id,
        "--on-duplicate", "ignore",
        user, relation, object,
    ]);
}

// ---------------------------------------------------------------------------
// OPA provisioning
// ---------------------------------------------------------------------------

/// Load the Rego policy into OPA via its bundle/data API.
async fn provision_opa(http: &reqwest::Client, opa_url: &str) {
    let policy = std::fs::read_to_string(rego_policy_file()).expect("read library.rego");
    let resp = http
        .put(format!("{opa_url}/v1/policies/library"))
        .header("Content-Type", "text/plain")
        .body(policy)
        .send()
        .await
        .expect("PUT /v1/policies/library");
    assert!(
        resp.status().is_success(),
        "failed to load Rego policy into OPA: {}",
        resp.status()
    );
}

/// Write a tuple directly into OPA's data store, bypassing the service layer.
/// Used to bootstrap system admins where no REST endpoint exists.
///
/// Layout mirrors OpaClient::tuple_path:
///   /v1/data/library/tuples/<obj_type>/<obj_id>/<relation>/<usr_type>/<usr_id>
pub async fn opa_write_tuple(
    http: &reqwest::Client,
    opa_url: &str,
    user: &str,
    relation: &str,
    object: &str,
) {
    fn encode(s: &str) -> String {
        s.replace('~', "~0").replace('/', "~1")
    }
    fn split_colon(s: &str) -> (&str, &str) {
        s.split_once(':').unwrap_or(("", s))
    }

    let (obj_type, obj_id) = split_colon(object);
    let (usr_type, usr_id) = if let Some(group_id) = user
        .strip_prefix("group:")
        .and_then(|s| s.strip_suffix("#member"))
    {
        ("group#member", group_id)
    } else {
        split_colon(user)
    };

    let path = format!(
        "{}/v1/data/library/tuples/{}/{}/{}/{}/{}",
        opa_url,
        encode(obj_type),
        encode(obj_id),
        encode(relation),
        encode(usr_type),
        encode(usr_id),
    );

    let resp = http.put(&path).json(&true).send().await.expect("PUT OPA tuple");
    assert!(
        resp.status().is_success(),
        "opa_write_tuple({user}, {relation}, {object}) failed: {}",
        resp.status()
    );
}

async fn wait_ready(http: &reqwest::Client, url: &str) {
    for attempt in 0..60 {
        if http.get(url).send().await.is_ok_and(|r| r.status().is_success()) {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        if attempt == 59 {
            panic!("{url} did not become ready after 60s");
        }
    }
}

// ---------------------------------------------------------------------------
// Stack bootstrap
// ---------------------------------------------------------------------------

pub async fn ensure_stack() -> &'static Stack {
    if let Some(s) = STACK.get() {
        return s;
    }

    let backend = std::env::var("AUTHZ_BACKEND").unwrap_or_else(|_| "openfga".into());

    let compose_path = compose_dir().join(match backend.as_str() {
        "opa" => "docker-compose.opa.yml",
        _     => "docker-compose.test.yml",
    });
    let compose_str = compose_path.to_str().expect("compose path");
    let http = reqwest::Client::new();

    let authz = match backend.as_str() {
        "opa" => {
            compose_up(compose_str, &["postgres", "opa"]);
            let opa_port = compose_port(compose_str, "opa", 8181);
            let opa_url = format!("http://127.0.0.1:{opa_port}");
            wait_ready(&http, &format!("{opa_url}/health")).await;
            provision_opa(&http, &opa_url).await;
            // Services need OPA_URL; no store/model IDs required.
            std::fs::write(authz_env_file(), format!("OPA_URL=http://opa:8181\n"))
                .expect("write openfga.env");
            AuthzSetup::Opa { url: opa_url }
        }
        _ => {
            compose_up(compose_str, &["postgres", "openfga-migrate", "openfga"]);
            let openfga_port = compose_port(compose_str, "openfga", 8080);
            let openfga_url = format!("http://127.0.0.1:{openfga_port}");
            wait_ready(&http, &format!("{openfga_url}/healthz")).await;
            let (store_id, model_id) = provision_openfga(&openfga_url);
            std::fs::write(
                authz_env_file(),
                format!("OPENFGA_STORE_ID={store_id}\nOPENFGA_MODEL_ID={model_id}\n"),
            )
            .expect("write openfga.env");
            AuthzSetup::OpenFga { url: openfga_url, store_id, model_id }
        }
    };

    compose_up(compose_str, &["user-service", "book-service", "review-service"]);
    let user_port   = compose_port(compose_str, "user-service", 3001);
    let book_port   = compose_port(compose_str, "book-service", 3002);
    let review_port = compose_port(compose_str, "review-service", 3003);

    let user_url   = format!("http://127.0.0.1:{user_port}");
    let book_url   = format!("http://127.0.0.1:{book_port}");
    let review_url = format!("http://127.0.0.1:{review_port}");

    wait_ready(&http, &format!("{user_url}/api-docs/openapi.json")).await;
    wait_ready(&http, &format!("{book_url}/api-docs/openapi.json")).await;
    wait_ready(&http, &format!("{review_url}/api-docs/openapi.json")).await;

    let _ = STACK.set(Stack {
        compose_file: compose_str.to_owned(),
        authz,
        user_url,
        book_url,
        review_url,
    });
    STACK.get().unwrap()
}
