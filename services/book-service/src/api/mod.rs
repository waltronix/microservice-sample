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
        .routes(routes!(
            handlers::books::create_book,
            handlers::books::list_books
        ))
        .routes(routes!(handlers::books::get_book, handlers::books::delete_book))
        .routes(routes!(handlers::books::grant_book_access))
        .with_state(state)
        .split_for_parts();

    let protected = router.layer(middleware::from_fn(require_caller_id));
    protected.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
}
