//! Get user use case.

use crate::application::dto::UserResponse;
use crate::domain::{self, UserId, UserRepository};

pub async fn get_user(
    repo: &dyn UserRepository,
    id: UserId,
) -> Result<UserResponse, domain::Error> {
    let user = repo.find_by_id(id).await?.ok_or(domain::Error::NotFound)?;
    Ok(user.into())
}
