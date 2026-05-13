//! Create user use case.

use crate::application::dto::{CreateUserRequest, UserResponse};
use crate::domain::{self, User, UserRepository};

pub async fn create_user(
    repo: &dyn UserRepository,
    request: CreateUserRequest,
) -> Result<UserResponse, domain::Error> {
    let user: User = request.try_into()?;
    repo.create(user.clone()).await?;
    Ok(user.into())
}
