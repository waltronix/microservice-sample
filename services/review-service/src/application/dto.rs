//! DTOs for the review-service API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::{self, BookId, Rating, Review, ReviewId, UserId};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateReviewRequest {
    pub book_id: Uuid,
    pub user_id: Uuid,
    pub rating: i16,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ReviewResponse {
    pub id: Uuid,
    pub book_id: Uuid,
    pub user_id: Uuid,
    pub rating: i16,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<CreateReviewRequest> for Review {
    type Error = domain::Error;

    fn try_from(value: CreateReviewRequest) -> Result<Self, Self::Error> {
        if value.body.trim().is_empty() {
            return Err(domain::Error::Validation("body cannot be empty".to_string()));
        }
        Ok(Review {
            id: ReviewId::new(Uuid::new_v4()),
            book_id: BookId::new(value.book_id),
            user_id: UserId::new(value.user_id),
            rating: Rating::new(value.rating)?,
            body: value.body,
            created_at: Utc::now(),
        })
    }
}

impl From<Review> for ReviewResponse {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_review_request_rejects_rating_zero() {
        let req = CreateReviewRequest {
            book_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            rating: 0,
            body: "good".to_string(),
        };
        assert!(matches!(
            Review::try_from(req),
            Err(domain::Error::Validation(_))
        ));
    }

    #[test]
    fn create_review_request_rejects_rating_six() {
        let req = CreateReviewRequest {
            book_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            rating: 6,
            body: "good".to_string(),
        };
        assert!(matches!(
            Review::try_from(req),
            Err(domain::Error::Validation(_))
        ));
    }

    #[test]
    fn create_review_request_rejects_empty_body() {
        let req = CreateReviewRequest {
            book_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            rating: 3,
            body: "".to_string(),
        };
        assert!(matches!(
            Review::try_from(req),
            Err(domain::Error::Validation(_))
        ));
    }

    #[test]
    fn create_review_request_assigns_fresh_uuid() {
        let req = CreateReviewRequest {
            book_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            rating: 4,
            body: "decent".to_string(),
        };
        let review = Review::try_from(req).unwrap();
        assert_ne!(review.id.into_uuid(), Uuid::nil());
    }
}
