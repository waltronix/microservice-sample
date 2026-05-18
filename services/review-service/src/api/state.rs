//! Shared Axum state.

use std::sync::Arc;

use authz::{HasAuthz, OpenFgaClient};

use crate::domain::ReviewRepository;

#[derive(Clone)]
pub struct AppState {
    pub review_repository: Arc<dyn ReviewRepository>,
    pub authz: Arc<OpenFgaClient>,
}

impl AppState {
    pub fn new(review_repository: Arc<dyn ReviewRepository>, authz: Arc<OpenFgaClient>) -> Self {
        Self {
            review_repository,
            authz,
        }
    }
}

impl HasAuthz for AppState {
    fn authz(&self) -> &Arc<OpenFgaClient> {
        &self.authz
    }
}
