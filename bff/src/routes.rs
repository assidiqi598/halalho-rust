use crate::{
    AppState,
    handlers::{
        auth_handlers::{login, logout, refresh, register, verify_email},
        test_handlers::test_publish,
    },
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

    let test_routes = Router::new().route("/rabbitmq_publish", post(test_publish));

    Router::new()
        .route("/", get(|| async { "Auth Service Running 🚀" }))
        .nest("/auth", auth_routes)
        .nest("/test", test_routes)
        .with_state(app_state)
}
