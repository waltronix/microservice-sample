//! Postgres implementation of `ReviewRepository`.

use async_trait::async_trait;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

use crate::domain::{self, BookId, Review, ReviewRepository};
use crate::infrastructure::database::models::ReviewRow;
use crate::infrastructure::database::schema::reviews;
use crate::infrastructure::database::Pool;

#[derive(Clone)]
pub struct PgReviewRepository {
    pool: Pool,
}

impl PgReviewRepository {
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
        DieselError::DatabaseError(DatabaseErrorKind::CheckViolation, info) => {
            domain::Error::Validation(info.message().to_string())
        }
        other => domain::Error::Infrastructure(other.to_string()),
    }
}

#[async_trait]
impl ReviewRepository for PgReviewRepository {
    async fn create(&self, review: Review) -> Result<(), domain::Error> {
        let row: ReviewRow = review.into();
        self.interact(move |conn| {
            diesel::insert_into(reviews::table)
                .values(&row)
                .execute(conn)
                .map(|_| ())
        })
        .await
    }

    async fn list_by_book(&self, book_id: BookId) -> Result<Vec<Review>, domain::Error> {
        let uuid = book_id.into_uuid();
        let rows = self
            .interact(move |conn| {
                reviews::table
                    .filter(reviews::book_id.eq(uuid))
                    .order(reviews::created_at.desc())
                    .select(ReviewRow::as_select())
                    .load::<ReviewRow>(conn)
            })
            .await?;
        rows.into_iter().map(Review::try_from).collect()
    }
}
