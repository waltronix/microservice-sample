//! OpenAPI document for user-service.

use utoipa::OpenApi;

use crate::api::handlers::groups::{AddMemberRequest, GroupResponse};
use crate::application::dto::{CreateUserRequest, UserResponse};

#[derive(OpenApi)]
#[openapi(
    info(title = "user-service", version = "0.1.0"),
    components(schemas(CreateUserRequest, UserResponse, GroupResponse, AddMemberRequest))
)]
pub struct ApiDoc;
