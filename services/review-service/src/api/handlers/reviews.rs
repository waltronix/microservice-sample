//! HTTP handlers for the `reviews` resource.

use axum::extract::{Extension, State};
use axum::http::StatusCode;
use axum::Json;
use library_core::UserId;
use authz::{BookReaderByBookId, ReviewWriter};

use crate::api::error::ApiError;
use crate::api::state::AppState;
use crate::application::dto::{CreateReviewRequest, ReviewResponse};
use crate::application::use_cases;

#[utoipa::path(
    post,
    path = "/reviews",
    request_body = CreateReviewRequest,
    responses(
        (status = 201, description = "Review created", body = ReviewResponse),
        (status = 400, description = "Validation error"),
        (status = 403, description = "Forbidden — caller does not have reader access to the book")
    )
)]
pub async fn create_review(
    State(state): State<AppState>,
    Extension(caller): Extension<UserId>,
    Json(request): Json<CreateReviewRequest>,
) -> Result<(StatusCode, Json<ReviewResponse>), ApiError> {
    // The book_id comes from the request body, not a path param, so we check
    // manually here rather than via a typed extractor.
    state
        .authz
        .require(
            &format!("user:{caller}"),
            "reader",
            &format!("book:{}", request.book_id),
        )
        .await?;
    let response = use_cases::create_review(state.review_repository.as_ref(), request).await?;
    state
        .authz
        .write_tuple(
            &format!("user:{caller}"),
            "owner",
            &format!("review:{}", response.id),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    get,
    path = "/books/{book_id}/reviews",
    params(("book_id" = Uuid, Path, description = "Book id to fetch reviews for")),
    responses(
        (status = 200, description = "Reviews for the book (newest first)", body = [ReviewResponse]),
        (status = 403, description = "Forbidden — caller does not have reader access to the book")
    )
)]
pub async fn list_reviews_by_book(
    BookReaderByBookId { book_id, .. }: BookReaderByBookId,
    State(state): State<AppState>,
) -> Result<Json<Vec<ReviewResponse>>, ApiError> {
    let response =
        use_cases::list_reviews_by_book(state.review_repository.as_ref(), book_id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    delete,
    path = "/reviews/{id}",
    params(("id" = Uuid, Path, description = "Review id")),
    responses(
        (status = 204, description = "Review deleted"),
        (status = 403, description = "Forbidden — caller does not have writer access to the review"),
        (status = 404, description = "Not found")
    )
)]
pub async fn delete_review(
    ReviewWriter { caller, review_id }: ReviewWriter,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {
    use_cases::delete_review(state.review_repository.as_ref(), review_id).await?;
    let _ = state
        .authz
        .delete_tuple(
            &format!("user:{caller}"),
            "owner",
            &format!("review:{review_id}"),
        )
        .await;
    Ok(StatusCode::NO_CONTENT)
}
