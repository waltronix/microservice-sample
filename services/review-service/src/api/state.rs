//! Shared Axum state.

use std::sync::Arc;

use crate::domain::ReviewRepository;

#[derive(Clone)]
pub struct AppState {
    pub review_repository: Arc<dyn ReviewRepository>,
}

impl AppState {
    pub fn new(review_repository: Arc<dyn ReviewRepository>) -> Self {
        Self { review_repository }
    }
}
