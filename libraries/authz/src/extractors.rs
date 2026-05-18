//! Typed authorization extractors.
//!
//! Each extractor performs an OpenFGA `check` during extraction, rejecting
//! with 403 before the handler body runs. All extractors require:
//!   1. The `require_caller_id` middleware to have run (provides `UserId` extension).
//!   2. The Axum state to implement [`HasAuthz`].

use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::path::RawPathParams;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use library_core::{BookId, ReviewId, UserId};
use serde_json::json;
use uuid::Uuid;

use crate::client::{AuthzError, OpenFgaClient};

// ---------------------------------------------------------------------------
// HasAuthz — implemented by each service's AppState
// ---------------------------------------------------------------------------

pub trait HasAuthz: Clone + Send + Sync + 'static {
    fn authz(&self) -> &Arc<OpenFgaClient>;
}

// ---------------------------------------------------------------------------
// Shared rejection type
// ---------------------------------------------------------------------------

pub struct AuthzRejection(AuthzError);

impl From<AuthzError> for AuthzRejection {
    fn from(e: AuthzError) -> Self {
        Self(e)
    }
}

impl IntoResponse for AuthzRejection {
    fn into_response(self) -> Response {
        let (status, msg) = match self.0 {
            AuthzError::Forbidden => (StatusCode::FORBIDDEN, "forbidden".to_string()),
            AuthzError::Infrastructure(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn caller_from_parts(parts: &Parts) -> Result<UserId, AuthzRejection> {
    parts
        .extensions
        .get::<UserId>()
        .copied()
        .ok_or(AuthzRejection(AuthzError::Forbidden))
}

async fn path_param(parts: &mut Parts, key: &str) -> Result<Uuid, AuthzRejection> {
    let raw = RawPathParams::from_request_parts(parts, &())
        .await
        .map_err(|_| AuthzRejection(AuthzError::Forbidden))?;
    raw.iter()
        .find(|(k, _)| &**k == key)
        .and_then(|(_, v)| Uuid::parse_str(v).ok())
        .ok_or(AuthzRejection(AuthzError::Forbidden))
}

// ---------------------------------------------------------------------------
// SystemAdmin — caller has `admin` on `system:library`
// ---------------------------------------------------------------------------

/// Asserts the caller is a system-level admin. No path param needed.
pub struct SystemAdmin(pub UserId);

#[async_trait]
impl<S: HasAuthz> FromRequestParts<S> for SystemAdmin {
    type Rejection = AuthzRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let caller = caller_from_parts(parts)?;
        state
            .authz()
            .require(&format!("user:{caller}"), "admin", "system:library")
            .await?;
        Ok(SystemAdmin(caller))
    }
}

// ---------------------------------------------------------------------------
// GroupAdmin — caller has `admin` on `group:{id}` (path param `id`)
// ---------------------------------------------------------------------------

/// Asserts the caller is an admin of the group at path param `{id}`.
pub struct GroupAdmin {
    pub caller: UserId,
    pub group_id: Uuid,
}

#[async_trait]
impl<S: HasAuthz> FromRequestParts<S> for GroupAdmin {
    type Rejection = AuthzRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let caller = caller_from_parts(parts)?;
        let group_id = path_param(parts, "id").await?;
        state
            .authz()
            .require(
                &format!("user:{caller}"),
                "admin",
                &format!("group:{group_id}"),
            )
            .await?;
        Ok(GroupAdmin { caller, group_id })
    }
}

// ---------------------------------------------------------------------------
// BookReader — caller has `reader` on `book:{id}` (path param `id`)
// ---------------------------------------------------------------------------

/// Asserts the caller has at least `reader` on the book at path param `{id}`.
pub struct BookReader {
    pub caller: UserId,
    pub book_id: BookId,
}

#[async_trait]
impl<S: HasAuthz> FromRequestParts<S> for BookReader {
    type Rejection = AuthzRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let caller = caller_from_parts(parts)?;
        let id = path_param(parts, "id").await?;
        state
            .authz()
            .require(&format!("user:{caller}"), "reader", &format!("book:{id}"))
            .await?;
        Ok(BookReader {
            caller,
            book_id: BookId::new(id),
        })
    }
}

// ---------------------------------------------------------------------------
// BookWriter — caller has `writer` on `book:{id}` (path param `id`)
// ---------------------------------------------------------------------------

/// Asserts the caller has at least `writer` on the book at path param `{id}`.
pub struct BookWriter {
    pub caller: UserId,
    pub book_id: BookId,
}

#[async_trait]
impl<S: HasAuthz> FromRequestParts<S> for BookWriter {
    type Rejection = AuthzRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let caller = caller_from_parts(parts)?;
        let id = path_param(parts, "id").await?;
        state
            .authz()
            .require(&format!("user:{caller}"), "writer", &format!("book:{id}"))
            .await?;
        Ok(BookWriter {
            caller,
            book_id: BookId::new(id),
        })
    }
}

// ---------------------------------------------------------------------------
// BookReaderByBookId — same as BookReader but uses path param `book_id`
// ---------------------------------------------------------------------------

/// Like [`BookReader`] but reads the `{book_id}` path segment.
/// Used by routes like `GET /books/{book_id}/reviews`.
pub struct BookReaderByBookId {
    pub caller: UserId,
    pub book_id: BookId,
}

#[async_trait]
impl<S: HasAuthz> FromRequestParts<S> for BookReaderByBookId {
    type Rejection = AuthzRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let caller = caller_from_parts(parts)?;
        let id = path_param(parts, "book_id").await?;
        state
            .authz()
            .require(&format!("user:{caller}"), "reader", &format!("book:{id}"))
            .await?;
        Ok(BookReaderByBookId {
            caller,
            book_id: BookId::new(id),
        })
    }
}

// ---------------------------------------------------------------------------
// ReviewWriter — caller has `writer` on `review:{id}` (path param `id`)
// ---------------------------------------------------------------------------

/// Asserts the caller has at least `writer` on the review at path param `{id}`.
pub struct ReviewWriter {
    pub caller: UserId,
    pub review_id: ReviewId,
}

#[async_trait]
impl<S: HasAuthz> FromRequestParts<S> for ReviewWriter {
    type Rejection = AuthzRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let caller = caller_from_parts(parts)?;
        let id = path_param(parts, "id").await?;
        state
            .authz()
            .require(
                &format!("user:{caller}"),
                "writer",
                &format!("review:{id}"),
            )
            .await?;
        Ok(ReviewWriter {
            caller,
            review_id: ReviewId::new(id),
        })
    }
}
