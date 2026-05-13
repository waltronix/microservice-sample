//! Book domain entity and value objects.

use chrono::{DateTime, Utc};

pub use library_core::BookId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Isbn(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IsbnError {
    #[error("ISBN cannot be empty")]
    Empty,
    #[error("ISBN must be 10 or 13 digits (hyphens allowed)")]
    InvalidLength,
    #[error("ISBN may only contain digits and hyphens")]
    InvalidCharacter,
}

impl Isbn {
    pub fn new(value: impl Into<String>) -> Result<Self, IsbnError> {
        let value = value.into();
        if value.is_empty() {
            return Err(IsbnError::Empty);
        }
        let mut digit_count = 0usize;
        for ch in value.chars() {
            if ch == '-' {
                continue;
            }
            if !ch.is_ascii_digit() {
                return Err(IsbnError::InvalidCharacter);
            }
            digit_count += 1;
        }
        if digit_count != 10 && digit_count != 13 {
            return Err(IsbnError::InvalidLength);
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
pub struct Title(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TitleError {
    #[error("title cannot be empty")]
    Empty,
}

impl Title {
    pub fn new(value: impl Into<String>) -> Result<Self, TitleError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(TitleError::Empty);
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
pub struct Book {
    pub id: BookId,
    pub isbn: Isbn,
    pub title: Title,
    pub author: String,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isbn_rejects_empty() {
        assert_eq!(Isbn::new(""), Err(IsbnError::Empty));
    }

    #[test]
    fn isbn_rejects_letters() {
        assert_eq!(Isbn::new("ABC1234567"), Err(IsbnError::InvalidCharacter));
    }

    #[test]
    fn isbn_rejects_wrong_length() {
        assert_eq!(Isbn::new("12345"), Err(IsbnError::InvalidLength));
    }

    #[test]
    fn isbn_accepts_10_digit_form() {
        assert!(Isbn::new("0306406152").is_ok());
    }

    #[test]
    fn isbn_accepts_13_digit_with_hyphens() {
        assert!(Isbn::new("978-3-16-148410-0").is_ok());
    }

    #[test]
    fn title_rejects_empty() {
        assert_eq!(Title::new(""), Err(TitleError::Empty));
    }

    #[test]
    fn title_rejects_whitespace_only() {
        assert_eq!(Title::new("   "), Err(TitleError::Empty));
    }

    #[test]
    fn title_accepts_non_empty() {
        assert_eq!(Title::new("Dune").unwrap().as_str(), "Dune");
    }
}
