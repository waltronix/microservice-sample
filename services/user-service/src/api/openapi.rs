//! OpenAPI document for user-service.

use utoipa::OpenApi;

use crate::application::dto::{CreateUserRequest, UserResponse};

#[derive(OpenApi)]
#[openapi(
    info(title = "user-service", version = "0.1.0"),
    components(schemas(CreateUserRequest, UserResponse))
)]
pub struct ApiDoc;
