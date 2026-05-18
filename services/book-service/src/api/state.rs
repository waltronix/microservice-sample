//! Shared Axum state.

use std::sync::Arc;

use authz::{HasAuthz, OpenFgaClient};

use crate::domain::BookRepository;

#[derive(Clone)]
pub struct AppState {
    pub book_repository: Arc<dyn BookRepository>,
    pub authz: Arc<OpenFgaClient>,
}

impl AppState {
    pub fn new(book_repository: Arc<dyn BookRepository>, authz: Arc<OpenFgaClient>) -> Self {
        Self {
            book_repository,
            authz,
        }
    }
}

impl HasAuthz for AppState {
    fn authz(&self) -> &Arc<OpenFgaClient> {
        &self.authz
    }
}
