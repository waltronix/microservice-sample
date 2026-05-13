//! List books use case.

use crate::application::dto::BookResponse;
use crate::domain::{self, BookRepository};

pub async fn list_books(repo: &dyn BookRepository) -> Result<Vec<BookResponse>, domain::Error> {
    let books = repo.list().await?;
    Ok(books.into_iter().map(BookResponse::from).collect())
}
