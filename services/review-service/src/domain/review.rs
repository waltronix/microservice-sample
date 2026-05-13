//! Review domain entity and value objects.

use chrono::{DateTime, Utc};

pub use library_core::{BookId, ReviewId, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rating(i16);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RatingError {
    #[error("rating must be between 1 and 5 inclusive (got {0})")]
    OutOfRange(i16),
}

impl Rating {
    pub fn new(value: i16) -> Result<Self, RatingError> {
        if !(1..=5).contains(&value) {
            return Err(RatingError::OutOfRange(value));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> i16 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Review {
    pub id: ReviewId,
    pub book_id: BookId,
    pub user_id: UserId,
    pub rating: Rating,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rating_rejects_zero() {
        assert_eq!(Rating::new(0), Err(RatingError::OutOfRange(0)));
    }

    #[test]
    fn rating_rejects_six() {
        assert_eq!(Rating::new(6), Err(RatingError::OutOfRange(6)));
    }

    #[test]
    fn rating_accepts_one_through_five() {
        for v in 1..=5 {
            assert!(Rating::new(v).is_ok());
        }
    }
}
