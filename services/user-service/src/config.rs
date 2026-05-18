//! Service configuration loaded from environment variables.

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
    pub openfga_url: String,
    pub openfga_store_id: String,
    pub openfga_model_id: String,
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
        let openfga_url = env::var("OPENFGA_URL")
            .map_err(|_| ConfigError::MissingVar("OPENFGA_URL"))?;
        let openfga_store_id = env::var("OPENFGA_STORE_ID")
            .map_err(|_| ConfigError::MissingVar("OPENFGA_STORE_ID"))?;
        let openfga_model_id = env::var("OPENFGA_MODEL_ID")
            .map_err(|_| ConfigError::MissingVar("OPENFGA_MODEL_ID"))?;
        Ok(Self {
            database_url,
            bind_addr,
            openfga_url,
            openfga_store_id,
            openfga_model_id,
        })
    }
}
