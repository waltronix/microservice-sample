//! Database infrastructure for book-service.

pub mod book_repository;
pub mod migrate;
pub mod models;
pub mod pool;
pub mod schema;

pub use pool::{build, Pool};
