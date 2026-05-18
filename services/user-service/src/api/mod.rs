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
    let (public_router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(
            handlers::users::create_user,
            handlers::users::list_users
        ))
        .routes(routes!(handlers::users::get_user))
        .with_state(state.clone())
        .split_for_parts();

    let (protected_router, _) = OpenApiRouter::new()
        .routes(routes!(handlers::groups::create_group))
        .routes(routes!(handlers::groups::add_member))
        .routes(routes!(handlers::groups::remove_member))
        .with_state(state)
        .split_for_parts();

    let protected = protected_router.layer(middleware::from_fn(require_caller_id));

    public_router
        .merge(protected)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
}
