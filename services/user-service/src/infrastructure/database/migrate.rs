//! Migration runner using `diesel_migrations::embed_migrations!`.

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use super::Pool;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Debug, thiserror::Error)]
pub enum MigrateError {
    #[error("failed to acquire database connection: {0}")]
    Pool(String),
    #[error("failed to run migrations: {0}")]
    Run(String),
}

pub async fn run(pool: &Pool) -> Result<(), MigrateError> {
    let conn = pool
        .get()
        .await
        .map_err(|e| MigrateError::Pool(e.to_string()))?;
    conn.interact(|conn| {
        conn.run_pending_migrations(MIGRATIONS)
            .map(|_| ())
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| MigrateError::Run(e.to_string()))?
    .map_err(MigrateError::Run)?;
    Ok(())
}
