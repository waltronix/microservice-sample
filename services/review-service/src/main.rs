//! review-service binary entrypoint.

use std::sync::Arc;

use tracing_subscriber::EnvFilter;

use review_service::api::{self, AppState};
use review_service::config::Config;
use review_service::infrastructure::database::{self, review_repository::PgReviewRepository};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("starting review-service");

    let config = Config::from_env()?;
    let pool = database::build(&config.database_url)?;
    database::migrate::run(&pool).await?;

    let repository = Arc::new(PgReviewRepository::new(pool));
    let authz: Arc<dyn authz::AuthzBackend> = match (
        config.openfga_store_id,
        config.openfga_model_id,
    ) {
        (Some(store_id), Some(model_id)) => {
            Arc::new(authz::OpenFgaClient::new(config.authz_url, store_id, model_id))
        }
        _ => Arc::new(authz::OpaClient::new(config.authz_url)),
    };

    let state = AppState::new(repository, authz);
    let router = api::router(state);

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    tracing::info!(addr = %config.bind_addr, "review-service listening");
    axum::serve(listener, router).await?;

    Ok(())
}
