use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::extract::Request;
use library_core::UserId;
use serde_json::json;
use uuid::Uuid;

/// Axum middleware that extracts `X-User-Id` from the request header and
/// inserts it as a `UserId` extension. Rejects with 403 if the header is
/// absent or not a valid UUID.
pub async fn require_caller_id(mut req: Request, next: Next) -> Response {
    let id = req
        .headers()
        .get("X-User-Id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok())
        .map(UserId::new);

    match id {
        Some(user_id) => {
            req.extensions_mut().insert(user_id);
            next.run(req).await
        }
        None => (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "missing or invalid X-User-Id header" })),
        )
            .into_response(),
    }
}

/// Extractor that retrieves the `UserId` inserted by [`require_caller_id`].
/// Panics at startup if the middleware was not applied, surfacing the
/// misconfiguration early.
pub struct CallerId(pub UserId);

pub struct MissingCallerId;

impl IntoResponse for MissingCallerId {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "caller id extension missing — require_caller_id middleware not applied" })),
        )
            .into_response()
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for CallerId {
    type Rejection = MissingCallerId;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<UserId>()
            .copied()
            .map(CallerId)
            .ok_or(MissingCallerId)
    }
}
