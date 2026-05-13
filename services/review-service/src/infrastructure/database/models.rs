//! Diesel row model for the `reviews` table and conversions to the domain.

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::{BookId, Rating, Review, ReviewId, UserId};

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::infrastructure::database::schema::reviews)]
pub struct ReviewRow {
    pub id: Uuid,
    pub book_id: Uuid,
    pub user_id: Uuid,
    pub rating: i16,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

impl From<Review> for ReviewRow {
    fn from(review: Review) -> Self {
        Self {
            id: review.id.into_uuid(),
            book_id: review.book_id.into_uuid(),
            user_id: review.user_id.into_uuid(),
            rating: review.rating.value(),
            body: review.body,
            created_at: review.created_at,
        }
    }
}

impl TryFrom<ReviewRow> for Review {
    type Error = crate::domain::Error;

    fn try_from(row: ReviewRow) -> Result<Self, Self::Error> {
        Ok(Review {
            id: ReviewId::new(row.id),
            book_id: BookId::new(row.book_id),
            user_id: UserId::new(row.user_id),
            rating: Rating::new(row.rating)?,
            body: row.body,
            created_at: row.created_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_round_trips_through_review_row() {
        let review = Review {
            id: ReviewId::new(Uuid::new_v4()),
            book_id: BookId::new(Uuid::new_v4()),
            user_id: UserId::new(Uuid::new_v4()),
            rating: Rating::new(4).unwrap(),
            body: "great read".to_string(),
            created_at: Utc::now(),
        };
        let row: ReviewRow = review.clone().into();
        let back: Review = row.try_into().unwrap();
        assert_eq!(review, back);
    }
}
