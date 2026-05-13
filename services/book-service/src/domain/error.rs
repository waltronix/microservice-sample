//! Domain error type for book-service.

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("book not found")]
    NotFound,
    #[error("validation error: {0}")]
    Validation(String),
    #[error("conflict")]
    Conflict,
    #[error("infrastructure error: {0}")]
    Infrastructure(String),
}

impl From<crate::domain::IsbnError> for Error {
    fn from(value: crate::domain::IsbnError) -> Self {
        Error::Validation(value.to_string())
    }
}

impl From<crate::domain::TitleError> for Error {
    fn from(value: crate::domain::TitleError) -> Self {
        Error::Validation(value.to_string())
    }
}
