use crate::{
    AppState,
    handlers::auth_handler::{login, logout, refresh, register, verify_email},
};
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let auth_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
        .route("/verify_email", get(verify_email));
    
    Router::new()
        .route("/", get(|| async { "Auth Service Running 🚀" }))
        .nest("/auth", auth_routes)
        .with_state(app_state)
}
