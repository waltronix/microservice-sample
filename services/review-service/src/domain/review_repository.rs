//! Repository trait for review persistence.

use async_trait::async_trait;

use super::{BookId, Error, Review, ReviewId};

#[async_trait]
pub trait ReviewRepository: Send + Sync + 'static {
    async fn create(&self, review: Review) -> Result<(), Error>;
    async fn find_by_id(&self, id: ReviewId) -> Result<Review, Error>;
    async fn list_by_book(&self, book_id: BookId) -> Result<Vec<Review>, Error>;
    async fn delete(&self, id: ReviewId) -> Result<(), Error>;
}
