use cucumber::{given, then};
use serde_json::Value;
use uuid::Uuid;

use crate::stack::ensure_stack;
use crate::world::E2eWorld;

#[given(expr = "user {string} creates a book")]
pub async fn step_create_book(world: &mut E2eWorld, user: String) {
    let stack = ensure_stack().await;
    // Each scenario needs a unique ISBN to avoid conflicts on the unique index.
    // The service accepts strings of 10–13 digits; we zero-pad the lower 32
    // bits of a fresh UUID to exactly 10 digits. Collision probability is
    // negligible across the small number of scenarios in one test run.
    let isbn = format!("{:010}", Uuid::new_v4().as_fields().0);
    let resp = world
        .http
        .post(format!("{}/books", stack.book_url))
        .header("X-User-Id", E2eWorld::user_id(&user).to_string())
        .json(&serde_json::json!({ "isbn": isbn, "title": "Dune", "author": "Frank Herbert" }))
        .send()
        .await
        .expect("create book");
    assert!(resp.status().is_success(), "create book failed: {}", resp.status());
    let body: Value = resp.json().await.expect("book json");
    world.book_id = Some(
        Uuid::parse_str(body["id"].as_str().expect("book id")).expect("parse uuid"),
    );
}

#[given(expr = "user {string} grants user {string} {string} access to the book")]
pub async fn step_grant_user_access(world: &mut E2eWorld, owner: String, grantee: String, relation: String) {
    let stack = ensure_stack().await;
    let book_id = world.book_id.expect("no book_id");
    let resp = world
        .http
        .put(format!("{}/books/{book_id}/access", stack.book_url))
        .header("X-User-Id", E2eWorld::user_id(&owner).to_string())
        .json(&serde_json::json!({
            "subject": format!("user:{}", E2eWorld::user_id(&grantee)),
            "relation": relation,
        }))
        .send()
        .await
        .expect("grant user access");
    assert!(resp.status().is_success(), "grant user access failed: {}", resp.status());
}

#[given(expr = "user {string} grants the group {string} access to the book")]
pub async fn step_grant_group_access(world: &mut E2eWorld, owner: String, relation: String) {
    let stack = ensure_stack().await;
    let book_id  = world.book_id.expect("no book_id");
    let group_id = world.group_id.expect("no group_id");
    let resp = world
        .http
        .put(format!("{}/books/{book_id}/access", stack.book_url))
        .header("X-User-Id", E2eWorld::user_id(&owner).to_string())
        .json(&serde_json::json!({
            "subject": format!("group:{group_id}#member"),
            "relation": relation,
        }))
        .send()
        .await
        .expect("grant group access");
    assert!(resp.status().is_success(), "grant group access failed: {}", resp.status());
}

#[then(expr = "user {string} can read the book")]
pub async fn assert_can_read_book(world: &mut E2eWorld, user: String) {
    let stack = ensure_stack().await;
    let book_id = world.book_id.expect("no book_id");
    world.assert_status(reqwest::Method::GET, format!("{}/books/{book_id}", stack.book_url), &user, 200).await;
}

#[then(expr = "user {string} cannot read the book")]
pub async fn assert_cannot_read_book(world: &mut E2eWorld, user: String) {
    let stack = ensure_stack().await;
    let book_id = world.book_id.expect("no book_id");
    world.assert_status(reqwest::Method::GET, format!("{}/books/{book_id}", stack.book_url), &user, 403).await;
}

#[then(expr = "user {string} can delete the book")]
pub async fn assert_can_delete_book(world: &mut E2eWorld, user: String) {
    let stack = ensure_stack().await;
    let book_id = world.book_id.expect("no book_id");
    world.assert_status(reqwest::Method::DELETE, format!("{}/books/{book_id}", stack.book_url), &user, 204).await;
}

#[then(expr = "user {string} cannot delete the book")]
pub async fn assert_cannot_delete_book(world: &mut E2eWorld, user: String) {
    let stack = ensure_stack().await;
    let book_id = world.book_id.expect("no book_id");
    world.assert_status(reqwest::Method::DELETE, format!("{}/books/{book_id}", stack.book_url), &user, 403).await;
}
