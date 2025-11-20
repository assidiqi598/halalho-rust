mod config {
    pub mod db;
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
    pub mod user;
}
mod services {
    pub mod user_service;
    pub mod auth_service;
}
mod error;
mod utils;

use crate::{config::db, services::user_service::UserService};
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
    println!("✅ Connected to MongoDB");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {
        user_service: UserService {
            db
        },
    }))
    .layer(cors);

    let port = std::env::var("BFF_PORT").expect("BFF_PORT is not set");

    let listener = tokio::net::TcpListener::bind("localhost:".to_string() + &port)
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
