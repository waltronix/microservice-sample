//! End-to-end BDD tests for the library authorization model.
//!
//! User identity: `X-User-Id` header, value is a UUID derived from the user's
//! name via UUID v5 so it is stable across steps without a creation step.
//!
//! System-admin bootstrap: there is no REST endpoint to grant the first admin,
//! so the "user X is a system admin" Given step writes the tuple directly via
//! the `fga` CLI, bypassing the service layer.

use std::process::Command;

use cucumber::World as _;

mod stack;
mod steps;
mod world;

use world::E2eWorld;

#[tokio::main]
async fn main() {
    let stack = stack::ensure_stack().await;
    E2eWorld::run("tests/features").await;
    // Tear down all containers and volumes. Runs even if scenarios fail because
    // cucumber exits via the normal return path, not panic, on step failures.
    let _ = Command::new("docker-compose")
        .args(["-f", &stack.compose_file, "down", "--volumes"])
        .status();
}
