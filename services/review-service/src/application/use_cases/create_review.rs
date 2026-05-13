//! Create review use case.

use crate::application::dto::{CreateReviewRequest, ReviewResponse};
use crate::domain::{self, Review, ReviewRepository};

pub async fn create_review(
    repo: &dyn ReviewRepository,
    request: CreateReviewRequest,
) -> Result<ReviewResponse, domain::Error> {
    let review: Review = request.try_into()?;
    repo.create(review.clone()).await?;
    Ok(review.into())
}
