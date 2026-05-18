//! Stack bootstrap — starts the full compose stack once per test binary run.
//!
//! Boot sequence (all via raw `docker-compose` CLI):
//!   1. `docker-compose.test.yml` — Postgres + OpenFGA infra services
//!   2. Provision OpenFGA store + model from `libraries/authz/fga/model.fga`
//!   3. `docker-compose.test.yml` again — user / book / review services
//!      (compose is idempotent; infra stays running, services start fresh)

use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

use serde_json::Value;

pub struct Stack {
    pub compose_file: String,
    pub openfga_url: String,
    pub store_id: String,
    pub model_id: String,
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

fn openfga_env_file() -> std::path::PathBuf {
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

pub async fn ensure_stack() -> &'static Stack {
    if let Some(s) = STACK.get() {
        return s;
    }

    let compose = compose_dir().join("docker-compose.test.yml");
    let compose_str = compose.to_str().expect("compose path");

    // Phase 1: infra — postgres + openfga.
    compose_up(compose_str, &["postgres", "openfga-migrate", "openfga"]);
    let openfga_port = compose_port(compose_str, "openfga", 8080);
    let openfga_url = format!("http://127.0.0.1:{openfga_port}");

    let http = reqwest::Client::new();
    wait_ready(&http, &format!("{openfga_url}/healthz")).await;

    // Phase 2: provision store + model; write env file so services can start.
    let (store_id, model_id) = provision_openfga(&openfga_url);
    std::fs::write(
        openfga_env_file(),
        format!("OPENFGA_STORE_ID={store_id}\nOPENFGA_MODEL_ID={model_id}\n"),
    )
    .expect("write openfga.env");

    // Phase 3: services — compose is idempotent, infra stays running.
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
        openfga_url,
        store_id,
        model_id,
        user_url,
        book_url,
        review_url,
    });
    STACK.get().unwrap()
}
