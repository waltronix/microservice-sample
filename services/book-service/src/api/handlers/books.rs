//! HTTP handlers for the `books` resource.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::api::error::ApiError;
use crate::api::state::AppState;
use crate::application::dto::{BookResponse, CreateBookRequest};
use crate::application::use_cases;
use crate::domain::BookId;

#[utoipa::path(
    post,
    path = "/books",
    request_body = CreateBookRequest,
    responses(
        (status = 201, description = "Book created", body = BookResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Conflict")
    )
)]
pub async fn create_book(
    State(state): State<AppState>,
    Json(request): Json<CreateBookRequest>,
) -> Result<(StatusCode, Json<BookResponse>), ApiError> {
    let response = use_cases::create_book(state.book_repository.as_ref(), request).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    get,
    path = "/books/{id}",
    params(("id" = Uuid, Path, description = "Book id")),
    responses(
        (status = 200, description = "Book found", body = BookResponse),
        (status = 404, description = "Not found")
    )
)]
pub async fn get_book(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<BookResponse>, ApiError> {
    let response = use_cases::get_book(state.book_repository.as_ref(), BookId::new(id)).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/books",
    responses(
        (status = 200, description = "List of books", body = [BookResponse])
    )
)]
pub async fn list_books(
    State(state): State<AppState>,
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
        (status = 404, description = "Not found")
    )
)]
pub async fn delete_book(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    use_cases::delete_book(state.book_repository.as_ref(), BookId::new(id)).await?;
    Ok(StatusCode::NO_CONTENT)
}
