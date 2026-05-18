use crate::domain::{self, ReviewId, ReviewRepository};

pub async fn delete_review(
    repo: &dyn ReviewRepository,
    id: ReviewId,
) -> Result<(), domain::Error> {
    repo.delete(id).await
}
