use cucumber::{given, when};
use serde_json::Value;
use uuid::Uuid;

use crate::stack::ensure_stack;
use crate::world::E2eWorld;

#[given(expr = "user {string} creates a group")]
#[when(expr = "user {string} creates a group")]
pub async fn step_create_group(world: &mut E2eWorld, user: String) {
    let stack = ensure_stack().await;
    let resp = world
        .http
        .post(format!("{}/groups", stack.user_url))
        .header("X-User-Id", E2eWorld::user_id(&user).to_string())
        .send()
        .await
        .expect("create group");
    world.last_status = resp.status().as_u16();
    if resp.status().is_success() {
        let body: Value = resp.json().await.expect("group response json");
        world.group_id = Some(
            Uuid::parse_str(body["id"].as_str().expect("group id")).expect("parse uuid"),
        );
    }
}

#[given(expr = "user {string} adds user {string} to the group")]
#[when(expr = "user {string} adds user {string} to the group")]
pub async fn step_add_member(world: &mut E2eWorld, caller: String, member: String) {
    let stack = ensure_stack().await;
    let group_id = world.group_id.expect("no group_id — create a group first");
    let resp = world
        .http
        .post(format!("{}/groups/{group_id}/members", stack.user_url))
        .header("X-User-Id", E2eWorld::user_id(&caller).to_string())
        .json(&serde_json::json!({ "user_id": E2eWorld::user_id(&member) }))
        .send()
        .await
        .expect("add member");
    world.last_status = resp.status().as_u16();
}

#[when(expr = "user {string} removes user {string} from the group")]
pub async fn step_remove_member(world: &mut E2eWorld, caller: String, member: String) {
    let stack = ensure_stack().await;
    let group_id = world.group_id.expect("no group_id");
    let member_id = E2eWorld::user_id(&member);
    let resp = world
        .http
        .delete(format!("{}/groups/{group_id}/members/{member_id}", stack.user_url))
        .header("X-User-Id", E2eWorld::user_id(&caller).to_string())
        .send()
        .await
        .expect("remove member");
    world.last_status = resp.status().as_u16();
}
