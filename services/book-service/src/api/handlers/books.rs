//! HTTP handlers for the `books` resource.

use axum::extract::{Extension, State};
use axum::http::StatusCode;
use axum::Json;
use library_core::UserId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use authz::{BookReader, BookWriter};

use crate::api::error::ApiError;
use crate::api::state::AppState;
use crate::application::dto::{BookResponse, CreateBookRequest};
use crate::application::use_cases;

#[derive(Deserialize, ToSchema)]
pub struct GrantAccessRequest {
    /// Subject in OpenFGA format: "user:<uuid>" or "group:<uuid>#member"
    pub subject: String,
    /// One of: "reader", "writer"
    pub relation: String,
}

#[derive(Serialize, ToSchema)]
pub struct GrantAccessResponse {}

#[utoipa::path(
    post,
    path = "/books",
    request_body = CreateBookRequest,
    responses(
        (status = 201, description = "Book created", body = BookResponse),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Conflict")
    )
)]
pub async fn create_book(
    State(state): State<AppState>,
    Extension(caller): Extension<UserId>,
    Json(request): Json<CreateBookRequest>,
) -> Result<(StatusCode, Json<BookResponse>), ApiError> {
    let response = use_cases::create_book(state.book_repository.as_ref(), request).await?;
    state
        .authz
        .write_tuple(
            &format!("user:{caller}"),
            "owner",
            &format!("book:{}", response.id),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    get,
    path = "/books/{id}",
    params(("id" = Uuid, Path, description = "Book id")),
    responses(
        (status = 200, description = "Book found", body = BookResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found")
    )
)]
pub async fn get_book(
    BookReader { book_id, .. }: BookReader,
    State(state): State<AppState>,
) -> Result<Json<BookResponse>, ApiError> {
    let response = use_cases::get_book(state.book_repository.as_ref(), book_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/books",
    responses(
        (status = 200, description = "List of books", body = [BookResponse]),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn list_books(
    State(state): State<AppState>,
    Extension(_caller): Extension<UserId>,
) -> Result<Json<Vec<BookResponse>>, ApiError> {
    let response = use_cases::list_books(state.book_repository.as_ref()).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/books/{id}",
    params(("id" = Uuid, Path, description = "Book id")),
    responses(
        (status = 204, description = "Book deleted"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not found")
    )
)]
pub async fn delete_book(
    BookWriter { caller, book_id }: BookWriter,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {
    use_cases::delete_book(state.book_repository.as_ref(), book_id).await?;
    let _ = state
        .authz
        .delete_tuple(
            &format!("user:{caller}"),
            "owner",
            &format!("book:{book_id}"),
        )
        .await;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    put,
    path = "/books/{id}/access",
    params(("id" = Uuid, Path, description = "Book id")),
    request_body = GrantAccessRequest,
    responses(
        (status = 204, description = "Access granted"),
        (status = 403, description = "Forbidden — caller does not have writer access"),
        (status = 404, description = "Not found")
    )
)]
pub async fn grant_book_access(
    BookWriter { book_id, .. }: BookWriter,
    State(state): State<AppState>,
    Json(req): Json<GrantAccessRequest>,
) -> Result<StatusCode, ApiError> {
    if req.relation != "reader" && req.relation != "writer" {
        return Err(ApiError::Validation(
            "relation must be 'reader' or 'writer'".into(),
        ));
    }
    state
        .authz
        .write_tuple(&req.subject, &req.relation, &format!("book:{book_id}"))
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
