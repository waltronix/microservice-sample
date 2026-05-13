//! DTOs for the user-service API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::{self, Email, User, UserId};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<CreateUserRequest> for User {
    type Error = domain::Error;

    fn try_from(value: CreateUserRequest) -> Result<Self, Self::Error> {
        if value.display_name.is_empty() {
            return Err(domain::Error::Validation(
                "display_name cannot be empty".to_string(),
            ));
        }
        Ok(User {
            id: UserId::new(Uuid::new_v4()),
            email: Email::new(value.email)?,
            display_name: value.display_name,
            created_at: Utc::now(),
        })
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.into_uuid(),
            email: user.email.into_string(),
            display_name: user.display_name,
            created_at: user.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_user_request_rejects_invalid_email() {
        let req = CreateUserRequest {
            email: "no-at-sign".to_string(),
            display_name: "Alice".to_string(),
        };
        assert!(matches!(
            User::try_from(req),
            Err(domain::Error::Validation(_))
        ));
    }

    #[test]
    fn create_user_request_assigns_fresh_uuid() {
        let req = CreateUserRequest {
            email: "alice@example.com".to_string(),
            display_name: "Alice".to_string(),
        };
        let user = User::try_from(req).unwrap();
        assert_ne!(user.id.into_uuid(), Uuid::nil());
    }
}
