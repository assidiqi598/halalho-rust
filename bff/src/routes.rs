use crate::{
    AppState,
    handlers::auth_handler::{login, logout, refresh, register},
};
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(|| async { "Auth Service Running 🚀" }))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
        .with_state(app_state)
}
