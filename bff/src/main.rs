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
    pub mod email_verif_token;
}
mod services {
    pub mod auth_service;
    pub mod email_service;
    pub mod refresh_token_service;
    pub mod storage_service;
    pub mod user_service;
    pub mod email_verif_token_service;
}
mod types {
    pub mod app_state;
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
        user_service::UserService, email_verif_token_service::VerifEmailTokenService,
    },
    types::app_state::AppState,
};
use axum::{
    extract::Request,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
};
use dotenvy::dotenv;
use routes::create_router;
use std::{env::var, sync::Arc};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultOnRequest, DefaultOnResponse, MakeSpan, TraceLayer},
};
use tracing::Span;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracing();

    let db = db::connect_db().await.unwrap();
    tracing::info!("✅ Connected to MongoDB");

    let r2_client = r2::connect_r2().await.unwrap();
    tracing::info!("✅ Connected to R2");

    let frontend = var("FRONTEND_URL").expect("FRONTEND_URL missing");

    let cors = CorsLayer::new()
        .allow_origin(frontend.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {
        user_service: UserService::new(db.clone()),
        refresh_token_service: RefreshTokenService::new(db.clone()),
        auth_service: AuthService::new(),
        storage_service: StorageService::new(r2_client),
        email_service: EmailService::new(),
        verif_email_token_service: VerifEmailTokenService::new(db),
    }))
    .layer(cors)
    .layer(init_req_tracer())
    .layer(PropagateRequestIdLayer::x_request_id())
    .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid));

    let port = var("BFF_PORT").expect("BFF_PORT is not set");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.unwrap();
    tracing::warn!("Shutdown signal received!");
}

fn init_tracing() {
    let fmt_layer = fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_target(false)
        .with_ansi(false)
        .with_timer(fmt::time::UtcTime::rfc_3339());

    registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=info", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(fmt_layer)
        .init();
}

fn init_req_tracer()
-> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, MakeSpanWithRequestId, DefaultOnRequest> {
    TraceLayer::new_for_http()
        .make_span_with(MakeSpanWithRequestId)
        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO))
}

#[derive(Clone)]
struct MakeSpanWithRequestId;

impl<B> MakeSpan<B> for MakeSpanWithRequestId {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let req_id = request
            .headers()
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("-");

        tracing::info_span!(
            "request",
            method = %request.method(),
            path = %request.uri().path(),
            request_id = %req_id
        )
    }
}
