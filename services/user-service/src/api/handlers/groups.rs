//! HTTP handlers for group management.
//!
//! Groups are not persisted in Postgres — OpenFGA is the source of truth.
//! Group UUIDs are generated here and returned to the caller.

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use authz::{GroupAdmin, SystemAdmin};

use crate::api::error::ApiError;
use crate::api::state::AppState;

#[derive(Serialize, ToSchema)]
pub struct GroupResponse {
    pub id: Uuid,
}

#[derive(Deserialize, ToSchema)]
pub struct AddMemberRequest {
    pub user_id: Uuid,
}

#[utoipa::path(
    post,
    path = "/groups",
    responses(
        (status = 201, description = "Group created", body = GroupResponse),
        (status = 403, description = "Forbidden — caller is not a system admin")
    )
)]
pub async fn create_group(
    SystemAdmin(caller): SystemAdmin,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<GroupResponse>), ApiError> {
    let group_id = Uuid::new_v4();
    state
        .authz
        .write_tuple(
            &format!("user:{caller}"),
            "admin",
            &format!("group:{group_id}"),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(GroupResponse { id: group_id })))
}

#[utoipa::path(
    post,
    path = "/groups/{id}/members",
    params(("id" = Uuid, Path, description = "Group id")),
    request_body = AddMemberRequest,
    responses(
        (status = 204, description = "Member added"),
        (status = 403, description = "Forbidden — caller is not an admin of the group")
    )
)]
pub async fn add_member(
    GroupAdmin { group_id, .. }: GroupAdmin,
    State(state): State<AppState>,
    Json(req): Json<AddMemberRequest>,
) -> Result<StatusCode, ApiError> {
    state
        .authz
        .write_tuple(
            &format!("user:{}", req.user_id),
            "member",
            &format!("group:{group_id}"),
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    delete,
    path = "/groups/{id}/members/{user_id}",
    params(
        ("id" = Uuid, Path, description = "Group id"),
        ("user_id" = Uuid, Path, description = "User id to remove")
    ),
    responses(
        (status = 204, description = "Member removed"),
        (status = 403, description = "Forbidden — caller is not an admin of the group")
    )
)]
pub async fn remove_member(
    GroupAdmin { group_id, .. }: GroupAdmin,
    State(state): State<AppState>,
    // user_id comes from path — extracted separately since GroupAdmin only reads `id`
    axum::extract::Path((_group_id, user_id)): axum::extract::Path<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    state
        .authz
        .delete_tuple(
            &format!("user:{user_id}"),
            "member",
            &format!("group:{group_id}"),
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
