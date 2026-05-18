//! Shared Axum state.

use std::sync::Arc;

use authz::{AuthzBackend, HasAuthz};

use crate::domain::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub authz: Arc<dyn AuthzBackend>,
}

impl AppState {
    pub fn new(user_repository: Arc<dyn UserRepository>, authz: Arc<dyn AuthzBackend>) -> Self {
        Self { user_repository, authz }
    }
}

impl HasAuthz for AppState {
    fn authz(&self) -> &Arc<dyn AuthzBackend> {
        &self.authz
    }
}
