//! HTTP handlers for the `reviews` resource.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::api::error::ApiError;
use crate::api::state::AppState;
use crate::application::dto::{CreateReviewRequest, ReviewResponse};
use crate::application::use_cases;
use crate::domain::BookId;

#[utoipa::path(
    post,
    path = "/reviews",
    request_body = CreateReviewRequest,
    responses(
        (status = 201, description = "Review created", body = ReviewResponse),
        (status = 400, description = "Validation error")
    )
)]
pub async fn create_review(
    State(state): State<AppState>,
    Json(request): Json<CreateReviewRequest>,
) -> Result<(StatusCode, Json<ReviewResponse>), ApiError> {
    let response = use_cases::create_review(state.review_repository.as_ref(), request).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    get,
    path = "/books/{book_id}/reviews",
    params(("book_id" = Uuid, Path, description = "Book id to fetch reviews for")),
    responses(
        (status = 200, description = "Reviews for the book (newest first)", body = [ReviewResponse])
    )
)]
pub async fn list_reviews_by_book(
    State(state): State<AppState>,
    Path(book_id): Path<Uuid>,
) -> Result<Json<Vec<ReviewResponse>>, ApiError> {
    let response =
        use_cases::list_reviews_by_book(state.review_repository.as_ref(), BookId::new(book_id))
            .await?;
    Ok(Json(response))
}
