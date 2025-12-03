mod config {
    pub mod db;
    pub mod r2;
}
mod routes;
mod handlers {
    pub mod auth_handler;
}
mod dtos {
    pub mod auth_dto;
    pub mod general_res_dto;
}
mod models {
    pub mod refresh_token;
    pub mod user;
    pub mod verif_email_token;
}
mod services {
    pub mod auth_service;
    pub mod email_service;
    pub mod refresh_token_service;
    pub mod storage_service;
    pub mod user_service;
    pub mod verif_email_token_service;
}
mod types {
    pub mod claims;
    pub mod email;
    pub mod error;
    pub mod keys;
    pub mod refresh_claims;
    pub mod verify_email;
}
mod utils {
    pub mod datetime;
    pub mod db_util;
}

use crate::{
    config::{db, r2},
    services::{
        auth_service::AuthService, email_service::EmailService,
        refresh_token_service::RefreshTokenService, storage_service::StorageService,
        user_service::UserService, verif_email_token_service::VerifEmailTokenService,
    },
};
use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use dotenvy::dotenv;
use routes::create_router;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct AppState {
    pub user_service: UserService,
    pub refresh_token_service: RefreshTokenService,
    pub auth_service: AuthService,
    pub storage_service: StorageService,
    pub email_service: EmailService,
    pub verif_email_token_service: VerifEmailTokenService
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = db::connect_db().await.unwrap();
    tracing::debug!("✅ Connected to MongoDB");

    let r2_client = r2::connect_r2().await.unwrap();
    tracing::debug!("✅ Connected to R2");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {
        user_service: UserService::new(db.clone()),
        refresh_token_service: RefreshTokenService::new(db.clone()),
        auth_service: AuthService::new(),
        storage_service: StorageService::new(r2_client),
        email_service: EmailService::new(),
        verif_email_token_service: VerifEmailTokenService::new(db)
    }))
    .layer(cors);

    let port = std::env::var("BFF_PORT").expect("BFF_PORT is not set");

    let listener = tokio::net::TcpListener::bind("localhost:".to_owned() + &port)
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
