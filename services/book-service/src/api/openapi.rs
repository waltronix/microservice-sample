//! OpenAPI document for book-service.

use utoipa::OpenApi;

use crate::application::dto::{BookResponse, CreateBookRequest};

#[derive(OpenApi)]
#[openapi(
    info(title = "book-service", version = "0.1.0"),
    components(schemas(CreateBookRequest, BookResponse))
)]
pub struct ApiDoc;
