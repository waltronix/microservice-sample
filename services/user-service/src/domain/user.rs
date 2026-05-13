//! User domain entity and value objects.

use chrono::{DateTime, Utc};

pub use library_core::UserId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EmailError {
    #[error("email cannot be empty")]
    Empty,
    #[error("email must contain '@'")]
    MissingAtSign,
}

impl Email {
    pub fn new(value: impl Into<String>) -> Result<Self, EmailError> {
        let value = value.into();
        if value.is_empty() {
            return Err(EmailError::Empty);
        }
        if !value.contains('@') {
            return Err(EmailError::MissingAtSign);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_rejects_empty_string() {
        assert_eq!(Email::new(""), Err(EmailError::Empty));
    }

    #[test]
    fn email_rejects_missing_at_sign() {
        assert_eq!(Email::new("not-an-email"), Err(EmailError::MissingAtSign));
    }

    #[test]
    fn email_accepts_valid_address() {
        let email = Email::new("alice@example.com").unwrap();
        assert_eq!(email.as_str(), "alice@example.com");
    }
}
