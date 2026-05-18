//! Shared Axum state.

use std::sync::Arc;

use authz::{HasAuthz, OpenFgaClient};

use crate::domain::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub authz: Arc<OpenFgaClient>,
}

impl AppState {
    pub fn new(user_repository: Arc<dyn UserRepository>, authz: Arc<OpenFgaClient>) -> Self {
        Self {
            user_repository,
            authz,
        }
    }
}

impl HasAuthz for AppState {
    fn authz(&self) -> &Arc<OpenFgaClient> {
        &self.authz
    }
}
