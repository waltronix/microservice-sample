//! Pure domain layer for user-service.

mod error;
mod user;
mod user_repository;

pub use error::Error;
pub use user::{Email, EmailError, User, UserId};
pub use user_repository::UserRepository;
