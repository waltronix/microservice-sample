//! HTTP handlers for the `users` resource.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::api::error::ApiError;
use crate::api::state::AppState;
use crate::application::dto::{CreateUserRequest, UserResponse};
use crate::application::use_cases;
use crate::domain::UserId;

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Conflict")
    )
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    let response = use_cases::create_user(state.user_repository.as_ref(), request).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(("id" = Uuid, Path, description = "User id")),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "Not found")
    )
)]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    let response = use_cases::get_user(state.user_repository.as_ref(), UserId::new(id)).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List of users", body = [UserResponse])
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    let response = use_cases::list_users(state.user_repository.as_ref()).await?;
    Ok(Json(response))
}
