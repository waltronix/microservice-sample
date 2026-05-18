//! book-service binary entrypoint.

use std::sync::Arc;

use authz::OpenFgaClient;
use tracing_subscriber::EnvFilter;

use book_service::api::{self, AppState};
use book_service::config::Config;
use book_service::infrastructure::database::{self, book_repository::PgBookRepository};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("starting book-service");

    let config = Config::from_env()?;
    let pool = database::build(&config.database_url)?;
    database::migrate::run(&pool).await?;

    let repository = Arc::new(PgBookRepository::new(pool));
    let authz = Arc::new(OpenFgaClient::new(
        config.openfga_url,
        config.openfga_store_id,
        config.openfga_model_id,
    ));
    let state = AppState::new(repository, authz);
    let router = api::router(state);

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    tracing::info!(addr = %config.bind_addr, "book-service listening");
    axum::serve(listener, router).await?;

    Ok(())
}
