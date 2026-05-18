use crate::application::dto::ReviewResponse;
use crate::domain::{self, ReviewId, ReviewRepository};

pub async fn get_review(
    repo: &dyn ReviewRepository,
    id: ReviewId,
) -> Result<ReviewResponse, domain::Error> {
    let review = repo.find_by_id(id).await?;
    Ok(review.into())
}
