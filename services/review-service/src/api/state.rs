//! Shared Axum state.

use std::sync::Arc;

use authz::{AuthzBackend, HasAuthz};

use crate::domain::ReviewRepository;

#[derive(Clone)]
pub struct AppState {
    pub review_repository: Arc<dyn ReviewRepository>,
    pub authz: Arc<dyn AuthzBackend>,
}

impl AppState {
    pub fn new(review_repository: Arc<dyn ReviewRepository>, authz: Arc<dyn AuthzBackend>) -> Self {
        Self { review_repository, authz }
    }
}

impl HasAuthz for AppState {
    fn authz(&self) -> &Arc<dyn AuthzBackend> {
        &self.authz
    }
}
