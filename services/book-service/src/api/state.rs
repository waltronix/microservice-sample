//! Shared Axum state.

use std::sync::Arc;

use crate::domain::BookRepository;

#[derive(Clone)]
pub struct AppState {
    pub book_repository: Arc<dyn BookRepository>,
}

impl AppState {
    pub fn new(book_repository: Arc<dyn BookRepository>) -> Self {
        Self { book_repository }
    }
}
