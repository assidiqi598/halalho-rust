use axum::{Router, routing::{get, post}};
use std::sync::Arc;
use crate::{AppState, handlers::auth_handler::{login, logout, register}};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .route("/", get(|| async { "Auth Service Running 🚀" }))
    .route("/register", post(register))
    .route("/login", post(login))
    .route("/logout", post(logout))
    .with_state(app_state)
}