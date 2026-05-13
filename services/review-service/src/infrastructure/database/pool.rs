//! deadpool-diesel connection pool.

use deadpool_diesel::postgres::{Manager, Runtime};

pub type Pool = deadpool_diesel::postgres::Pool;

#[derive(Debug, thiserror::Error)]
pub enum PoolError {
    #[error("failed to build pool: {0}")]
    Build(String),
}

pub fn build(database_url: &str) -> Result<Pool, PoolError> {
    let manager = Manager::new(database_url.to_string(), Runtime::Tokio1);
    Pool::builder(manager)
        .build()
        .map_err(|e| PoolError::Build(e.to_string()))
}
