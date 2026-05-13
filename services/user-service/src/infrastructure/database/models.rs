//! Diesel row model for the `users` table and conversions to the domain.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::{Email, User, UserId};

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::infrastructure::database::schema::users)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserRow {
    fn from(user: User) -> Self {
        Self {
            id: user.id.into_uuid(),
            email: user.email.into_string(),
            display_name: user.display_name,
            created_at: user.created_at,
        }
    }
}

impl TryFrom<UserRow> for User {
    type Error = crate::domain::Error;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        Ok(User {
            id: UserId::new(row.id),
            email: Email::new(row.email)?,
            display_name: row.display_name,
            created_at: row.created_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_round_trips_through_user_row() {
        let user = User {
            id: UserId::new(Uuid::new_v4()),
            email: Email::new("alice@example.com").unwrap(),
            display_name: "Alice".to_string(),
            created_at: Utc::now(),
        };
        let row: UserRow = user.clone().into();
        let back: User = row.try_into().unwrap();
        assert_eq!(user, back);
    }
}
