use cucumber::{given, then, when};
use serde_json::Value;
use uuid::Uuid;

use crate::stack::ensure_stack;
use crate::world::E2eWorld;

#[given(expr = "user {string} creates a review for the book")]
#[when(expr = "user {string} creates a review for the book")]
pub async fn step_create_review(world: &mut E2eWorld, user: String) {
    let stack = ensure_stack().await;
    let book_id = world.book_id.expect("no book_id");
    let resp = world
        .http
        .post(format!("{}/reviews", stack.review_url))
        .header("X-User-Id", E2eWorld::user_id(&user).to_string())
        .json(&serde_json::json!({
            "book_id": book_id,
            "user_id": E2eWorld::user_id(&user),
            "rating": 4,
            "body": "Great book",
        }))
        .send()
        .await
        .expect("create review");
    world.last_status = resp.status().as_u16();
    if resp.status().is_success() {
        let body: Value = resp.json().await.expect("review json");
        world.review_id = Some(
            Uuid::parse_str(body["id"].as_str().expect("review id")).expect("parse uuid"),
        );
    }
}

#[when(expr = "user {string} deletes the review")]
pub async fn step_delete_review(world: &mut E2eWorld, user: String) {
    let stack = ensure_stack().await;
    let review_id = world.review_id.expect("no review_id");
    let resp = world
        .http
        .delete(format!("{}/reviews/{review_id}", stack.review_url))
        .header("X-User-Id", E2eWorld::user_id(&user).to_string())
        .send()
        .await
        .expect("delete review");
    world.last_status = resp.status().as_u16();
}

#[then(expr = "the response status is {int}")]
pub async fn assert_status(world: &mut E2eWorld, expected: u16) {
    assert_eq!(world.last_status, expected, "expected status {expected}, got {}", world.last_status);
}

#[then(expr = "user {string} can list reviews for the book")]
pub async fn assert_can_list_reviews(world: &mut E2eWorld, user: String) {
    let stack = ensure_stack().await;
    let book_id = world.book_id.expect("no book_id");
    world.assert_status(reqwest::Method::GET, format!("{}/books/{book_id}/reviews", stack.review_url), &user, 200).await;
}

#[then(expr = "user {string} cannot list reviews for the book")]
pub async fn assert_cannot_list_reviews(world: &mut E2eWorld, user: String) {
    let stack = ensure_stack().await;
    let book_id = world.book_id.expect("no book_id");
    world.assert_status(reqwest::Method::GET, format!("{}/books/{book_id}/reviews", stack.review_url), &user, 403).await;
}
