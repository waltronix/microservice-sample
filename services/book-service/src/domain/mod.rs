//! Pure domain layer for book-service.

mod book;
mod book_repository;
mod error;

pub use book::{Book, BookId, Isbn, IsbnError, Title, TitleError};
pub use book_repository::BookRepository;
pub use error::Error;
