//! Pure domain layer for review-service.

mod error;
mod review;
mod review_repository;

pub use error::Error;
pub use review::{BookId, Rating, RatingError, Review, ReviewId, UserId};
pub use review_repository::ReviewRepository;
