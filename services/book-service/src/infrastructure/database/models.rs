//! Diesel row model for the `books` table and conversions to the domain.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::{Book, BookId, Isbn, Title};

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::infrastructure::database::schema::books)]
pub struct BookRow {
    pub id: Uuid,
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
}

impl From<Book> for BookRow {
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

impl TryFrom<BookRow> for Book {
    type Error = crate::domain::Error;

    fn try_from(row: BookRow) -> Result<Self, Self::Error> {
        Ok(Book {
            id: BookId::new(row.id),
            isbn: Isbn::new(row.isbn)?,
            title: Title::new(row.title)?,
            author: row.author,
            created_at: row.created_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_round_trips_through_book_row() {
        let book = Book {
            id: BookId::new(Uuid::new_v4()),
            isbn: Isbn::new("978-3-16-148410-0").unwrap(),
            title: Title::new("Dune").unwrap(),
            author: "Frank Herbert".to_string(),
            created_at: Utc::now(),
        };
        let row: BookRow = book.clone().into();
        let back: Book = row.try_into().unwrap();
        assert_eq!(book, back);
    }
}
