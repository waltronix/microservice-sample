//! List reviews by book use case.

use crate::application::dto::ReviewResponse;
use crate::domain::{self, BookId, ReviewRepository};

pub async fn list_reviews_by_book(
    repo: &dyn ReviewRepository,
    book_id: BookId,
) -> Result<Vec<ReviewResponse>, domain::Error> {
    let reviews = repo.list_by_book(book_id).await?;
    Ok(reviews.into_iter().map(ReviewResponse::from).collect())
}
