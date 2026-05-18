//! Shared Axum state.

use std::sync::Arc;

use authz::{AuthzBackend, HasAuthz};

use crate::domain::BookRepository;

#[derive(Clone)]
pub struct AppState {
    pub book_repository: Arc<dyn BookRepository>,
    pub authz: Arc<dyn AuthzBackend>,
}

impl AppState {
    pub fn new(book_repository: Arc<dyn BookRepository>, authz: Arc<dyn AuthzBackend>) -> Self {
        Self { book_repository, authz }
    }
}

impl HasAuthz for AppState {
    fn authz(&self) -> &Arc<dyn AuthzBackend> {
        &self.authz
    }
}
