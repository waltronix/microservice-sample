//! Postgres implementation of `BookRepository`.

use async_trait::async_trait;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

use crate::domain::{self, Book, BookId, BookRepository};
use crate::infrastructure::database::models::BookRow;
use crate::infrastructure::database::schema::books;
use crate::infrastructure::database::Pool;

#[derive(Clone)]
pub struct PgBookRepository {
    pool: Pool,
}

impl PgBookRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    async fn interact<F, R>(&self, f: F) -> Result<R, domain::Error>
    where
        F: FnOnce(&mut diesel::pg::PgConnection) -> Result<R, DieselError> + Send + 'static,
        R: Send + 'static,
    {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| domain::Error::Infrastructure(e.to_string()))?;
        conn.interact(move |conn| f(conn))
            .await
            .map_err(|e| domain::Error::Infrastructure(e.to_string()))?
            .map_err(map_diesel_error)
    }
}

fn map_diesel_error(error: DieselError) -> domain::Error {
    match error {
        DieselError::NotFound => domain::Error::NotFound,
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            domain::Error::Conflict
        }
        other => domain::Error::Infrastructure(other.to_string()),
    }
}

#[async_trait]
impl BookRepository for PgBookRepository {
    async fn create(&self, book: Book) -> Result<(), domain::Error> {
        let row: BookRow = book.into();
        self.interact(move |conn| {
            diesel::insert_into(books::table)
                .values(&row)
                .execute(conn)
                .map(|_| ())
        })
        .await
    }

    async fn find_by_id(&self, id: BookId) -> Result<Option<Book>, domain::Error> {
        let uuid = id.into_uuid();
        let row = self
            .interact(move |conn| {
                books::table
                    .find(uuid)
                    .select(BookRow::as_select())
                    .first::<BookRow>(conn)
                    .optional()
            })
            .await?;
        row.map(Book::try_from).transpose()
    }

    async fn list(&self) -> Result<Vec<Book>, domain::Error> {
        let rows = self
            .interact(|conn| {
                books::table
                    .select(BookRow::as_select())
                    .load::<BookRow>(conn)
            })
            .await?;
        rows.into_iter().map(Book::try_from).collect()
    }

    async fn delete(&self, id: BookId) -> Result<(), domain::Error> {
        let uuid = id.into_uuid();
        let deleted = self
            .interact(move |conn| diesel::delete(books::table.find(uuid)).execute(conn))
            .await?;
        if deleted == 0 {
            Err(domain::Error::NotFound)
        } else {
            Ok(())
        }
    }
}
