//! OpenAPI document for book-service.

use utoipa::OpenApi;

use crate::api::handlers::books::{GrantAccessRequest, GrantAccessResponse};
use crate::application::dto::{BookResponse, CreateBookRequest};

#[derive(OpenApi)]
#[openapi(
    info(title = "book-service", version = "0.1.0"),
    components(schemas(CreateBookRequest, BookResponse, GrantAccessRequest, GrantAccessResponse))
)]
pub struct ApiDoc;
