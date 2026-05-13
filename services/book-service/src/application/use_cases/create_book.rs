//! Create book use case.

use crate::application::dto::{BookResponse, CreateBookRequest};
use crate::domain::{self, Book, BookRepository};

pub async fn create_book(
    repo: &dyn BookRepository,
    request: CreateBookRequest,
) -> Result<BookResponse, domain::Error> {
    let book: Book = request.try_into()?;
    repo.create(book.clone()).await?;
    Ok(book.into())
}
