use crate::db::Database;
use crate::handlers::{health, hello};
use axum::{Router, routing::get};

pub fn create_routes(_db: Database) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/hello", get(hello))
}
