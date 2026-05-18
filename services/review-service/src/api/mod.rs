//! API layer: Axum router, handlers, error mapping, OpenAPI.

pub mod error;
pub mod handlers;
pub mod openapi;
pub mod state;

use axum::middleware;
use authz::require_caller_id;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

pub use state::AppState;

use self::openapi::ApiDoc;

pub fn router(state: AppState) -> axum::Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(handlers::reviews::create_review))
        .routes(routes!(handlers::reviews::list_reviews_by_book))
        .routes(routes!(handlers::reviews::delete_review))
        .with_state(state)
        .split_for_parts();

    let protected = router.layer(middleware::from_fn(require_caller_id));
    protected.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
}
