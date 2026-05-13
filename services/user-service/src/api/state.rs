//! Shared Axum state.

use std::sync::Arc;

use crate::domain::UserRepository;

#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
}

impl AppState {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}
