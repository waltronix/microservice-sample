//! Delete book use case.

use crate::domain::{self, BookId, BookRepository};

pub async fn delete_book(repo: &dyn BookRepository, id: BookId) -> Result<(), domain::Error> {
    repo.delete(id).await
}
