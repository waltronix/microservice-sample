//! Database infrastructure for user-service.

pub mod migrate;
pub mod models;
pub mod pool;
pub mod schema;
pub mod user_repository;

pub use pool::{build, Pool};
