//! Repository trait for book persistence.

use async_trait::async_trait;

use super::{Book, BookId, Error};

#[async_trait]
pub trait BookRepository: Send + Sync + 'static {
    async fn create(&self, book: Book) -> Result<(), Error>;
    async fn find_by_id(&self, id: BookId) -> Result<Option<Book>, Error>;
    async fn list(&self) -> Result<Vec<Book>, Error>;
    async fn delete(&self, id: BookId) -> Result<(), Error>;
}
