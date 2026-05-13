//! Domain error type for user-service.

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("user not found")]
    NotFound,
    #[error("validation error: {0}")]
    Validation(String),
    #[error("conflict")]
    Conflict,
    #[error("infrastructure error: {0}")]
    Infrastructure(String),
}

impl From<crate::domain::EmailError> for Error {
    fn from(value: crate::domain::EmailError) -> Self {
        Error::Validation(value.to_string())
    }
}
