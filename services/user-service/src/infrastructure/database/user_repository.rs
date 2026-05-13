//! Postgres implementation of `UserRepository`.

use async_trait::async_trait;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

use crate::domain::{self, User, UserId, UserRepository};
use crate::infrastructure::database::models::UserRow;
use crate::infrastructure::database::schema::users;
use crate::infrastructure::database::Pool;

#[derive(Clone)]
pub struct PgUserRepository {
    pool: Pool,
}

impl PgUserRepository {
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
impl UserRepository for PgUserRepository {
    async fn create(&self, user: User) -> Result<(), domain::Error> {
        let row: UserRow = user.into();
        self.interact(move |conn| {
            diesel::insert_into(users::table)
                .values(&row)
                .execute(conn)
                .map(|_| ())
        })
        .await
    }

    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, domain::Error> {
        let uuid = id.into_uuid();
        let row = self
            .interact(move |conn| {
                users::table
                    .find(uuid)
                    .select(UserRow::as_select())
                    .first::<UserRow>(conn)
                    .optional()
            })
            .await?;
        row.map(User::try_from).transpose()
    }

    async fn list(&self) -> Result<Vec<User>, domain::Error> {
        let rows = self
            .interact(|conn| {
                users::table
                    .select(UserRow::as_select())
                    .load::<UserRow>(conn)
            })
            .await?;
        rows.into_iter().map(User::try_from).collect()
    }
}
