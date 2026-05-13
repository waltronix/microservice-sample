//! Service configuration loaded from environment variables.

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing required environment variable: {0}")]
    MissingVar(&'static str),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let _ = dotenvy::dotenv();
        let database_url = env::var("USERS_DATABASE_URL")
            .map_err(|_| ConfigError::MissingVar("USERS_DATABASE_URL"))?;
        let bind_addr =
            env::var("USERS_BIND_ADDR").map_err(|_| ConfigError::MissingVar("USERS_BIND_ADDR"))?;
        Ok(Self {
            database_url,
            bind_addr,
        })
    }
}
