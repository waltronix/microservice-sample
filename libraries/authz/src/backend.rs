use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthzError {
    #[error("forbidden")]
    Forbidden,
    #[error("authorization service error: {0}")]
    Infrastructure(String),
}

/// Abstraction over an authorization backend.
///
/// Both `OpenFgaClient` and `OpaClient` implement this trait so the rest of
/// the library and all services can be written against it without knowing
/// which backend is active.
#[async_trait]
pub trait AuthzBackend: Send + Sync + 'static {
    async fn check(&self, user: &str, relation: &str, object: &str) -> Result<bool, AuthzError>;
    async fn write_tuple(&self, user: &str, relation: &str, object: &str) -> Result<(), AuthzError>;
    async fn delete_tuple(&self, user: &str, relation: &str, object: &str) -> Result<(), AuthzError>;

    /// Enforce a check, returning `Forbidden` if not allowed.
    async fn require(&self, user: &str, relation: &str, object: &str) -> Result<(), AuthzError> {
        if self.check(user, relation, object).await? {
            Ok(())
        } else {
            Err(AuthzError::Forbidden)
        }
    }
}
