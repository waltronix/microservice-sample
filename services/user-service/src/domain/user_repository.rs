//! Repository trait for user persistence.

use async_trait::async_trait;

use super::{Error, User, UserId};

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: User) -> Result<(), Error>;
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, Error>;
    async fn list(&self) -> Result<Vec<User>, Error>;
}
