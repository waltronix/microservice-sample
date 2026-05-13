//! API layer: Axum router, handlers, error mapping, OpenAPI.

pub mod error;
pub mod handlers;
pub mod openapi;
pub mod state;

use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

pub use state::AppState;

use self::openapi::ApiDoc;

pub fn router(state: AppState) -> axum::Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(
            handlers::users::create_user,
            handlers::users::list_users
        ))
        .routes(routes!(handlers::users::get_user))
        .with_state(state)
        .split_for_parts();

    router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
}
