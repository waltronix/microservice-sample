//! DTOs for the book-service API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::{self, Book, BookId, Isbn, Title};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateBookRequest {
    pub isbn: String,
    pub title: String,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BookResponse {
    pub id: Uuid,
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<CreateBookRequest> for Book {
    type Error = domain::Error;

    fn try_from(value: CreateBookRequest) -> Result<Self, Self::Error> {
        if value.author.trim().is_empty() {
            return Err(domain::Error::Validation(
                "author cannot be empty".to_string(),
            ));
        }
        Ok(Book {
            id: BookId::new(Uuid::new_v4()),
            isbn: Isbn::new(value.isbn)?,
            title: Title::new(value.title)?,
            author: value.author,
            created_at: Utc::now(),
        })
    }
}

impl From<Book> for BookResponse {
    fn from(book: Book) -> Self {
        Self {
            id: book.id.into_uuid(),
            isbn: book.isbn.into_string(),
            title: book.title.into_string(),
            author: book.author,
            created_at: book.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_book_request_rejects_invalid_isbn() {
        let req = CreateBookRequest {
            isbn: "not-an-isbn".to_string(),
            title: "Dune".to_string(),
            author: "Frank Herbert".to_string(),
        };
        assert!(matches!(
            Book::try_from(req),
            Err(domain::Error::Validation(_))
        ));
    }

    #[test]
    fn create_book_request_rejects_empty_author() {
        let req = CreateBookRequest {
            isbn: "0306406152".to_string(),
            title: "Dune".to_string(),
            author: "".to_string(),
        };
        assert!(matches!(
            Book::try_from(req),
            Err(domain::Error::Validation(_))
        ));
    }

    #[test]
    fn create_book_request_assigns_fresh_uuid() {
        let req = CreateBookRequest {
            isbn: "0306406152".to_string(),
            title: "Dune".to_string(),
            author: "Frank Herbert".to_string(),
        };
        let book = Book::try_from(req).unwrap();
        assert_ne!(book.id.into_uuid(), Uuid::nil());
    }
}
