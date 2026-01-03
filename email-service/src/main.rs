mod config {
    pub mod db;
    pub mod r2;
    pub mod rabbitmq;
}

mod dtos {
    pub mod user_id_event;
}

mod handlers {
    pub mod send_email_verif_handler;
}

mod models {
    pub mod email_verif_token;
}

mod services {
    pub mod email_service;
    pub mod email_verif_token_service;
    pub mod storage_service;
}

mod types {
    pub mod consumer;
    pub mod email;
    pub mod error;
    pub mod verify_email;
}

mod utils {
    pub mod datetime;
    pub mod db_util;
}

use std::sync::Arc;

use amqprs::channel::BasicConsumeArguments;
use tokio::sync::Notify;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

use config::{db, r2, rabbitmq};

use crate::{services::{email_service::EmailService, email_verif_token_service::VerifEmailTokenService, storage_service::StorageService}, types::consumer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let db = db::connect_db().await.unwrap();
    tracing::info!("✅ Connected to MongoDB");

    let r2_client = r2::connect_r2().await.unwrap();
    tracing::info!("✅ Connected to R2");

    let (_conn, channel, routing_key, queue_name) = rabbitmq::setup_rabbitmq_client().await;

    let args = BasicConsumeArguments::new(&queue_name, "email_consumer")
        .manual_ack(true)
        .finish();

    channel
        .basic_consume(
            consumer::MainServiceConsumer {
                verif_email_token_service: Arc::new(VerifEmailTokenService::new(db)),
                storage_service: Arc::new(StorageService::new(r2_client.clone())),
                email_service: Arc::new(EmailService::new()),
            },
            args,
        )
        .await
        .unwrap();

    tracing::info!("Start consuming {}", routing_key);
    let guard = Notify::new();
    guard.notified().await;
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
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(fmt_layer)
        .init();
}
