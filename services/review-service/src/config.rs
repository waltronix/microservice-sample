//! Service configuration loaded from environment variables.

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
    /// Base URL of the active authorization backend (OpenFGA or OPA).
    pub authz_url: String,
    /// Only required when `AUTHZ_BACKEND=openfga` (the default).
    pub openfga_store_id: Option<String>,
    pub openfga_model_id: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("missing required environment variable: {0}")]
    MissingVar(&'static str),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let _ = dotenvy::dotenv();

        let backend = env::var("AUTHZ_BACKEND").unwrap_or_else(|_| "openfga".into());

        let (authz_url, openfga_store_id, openfga_model_id) = if backend == "opa" {
            let url = env::var("OPA_URL").map_err(|_| ConfigError::MissingVar("OPA_URL"))?;
            (url, None, None)
        } else {
            let url = env::var("OPENFGA_URL")
                .map_err(|_| ConfigError::MissingVar("OPENFGA_URL"))?;
            let store_id = env::var("OPENFGA_STORE_ID")
                .map_err(|_| ConfigError::MissingVar("OPENFGA_STORE_ID"))?;
            let model_id = env::var("OPENFGA_MODEL_ID")
                .map_err(|_| ConfigError::MissingVar("OPENFGA_MODEL_ID"))?;
            (url, Some(store_id), Some(model_id))
        };

        Ok(Self {
            database_url: env::var("REVIEWS_DATABASE_URL")
                .map_err(|_| ConfigError::MissingVar("REVIEWS_DATABASE_URL"))?,
            bind_addr: env::var("REVIEWS_BIND_ADDR")
                .map_err(|_| ConfigError::MissingVar("REVIEWS_BIND_ADDR"))?,
            authz_url,
            openfga_store_id,
            openfga_model_id,
        })
    }
}
