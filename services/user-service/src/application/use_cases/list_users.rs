//! List users use case.

use crate::application::dto::UserResponse;
use crate::domain::{self, UserRepository};

pub async fn list_users(repo: &dyn UserRepository) -> Result<Vec<UserResponse>, domain::Error> {
    let users = repo.list().await?;
    Ok(users.into_iter().map(UserResponse::from).collect())
}
