//! OpenAPI document for review-service.

use utoipa::OpenApi;

use crate::application::dto::{CreateReviewRequest, ReviewResponse};

#[derive(OpenApi)]
#[openapi(
    info(title = "review-service", version = "0.1.0"),
    components(schemas(CreateReviewRequest, ReviewResponse))
)]
pub struct ApiDoc;
