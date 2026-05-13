//! Domain error type for review-service.

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("review not found")]
    NotFound,
    #[error("validation error: {0}")]
    Validation(String),
    #[error("conflict")]
    Conflict,
    #[error("infrastructure error: {0}")]
    Infrastructure(String),
}

impl From<crate::domain::RatingError> for Error {
    fn from(value: crate::domain::RatingError) -> Self {
        Error::Validation(value.to_string())
    }
}
