//! Get book use case.

use crate::application::dto::BookResponse;
use crate::domain::{self, BookId, BookRepository};

pub async fn get_book(
    repo: &dyn BookRepository,
    id: BookId,
) -> Result<BookResponse, domain::Error> {
    let book = repo.find_by_id(id).await?.ok_or(domain::Error::NotFound)?;
    Ok(book.into())
}
