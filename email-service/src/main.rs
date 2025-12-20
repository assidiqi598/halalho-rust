mod config {
    pub mod rabbitmq;
}

mod types {
    pub mod consumer;
    // pub mod email;
    // pub mod verify_email;
}

use tokio::sync::Notify;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

use config::rabbitmq;

#[tokio::main]
async fn main() {
    init_tracing();

    let (_conn, _channel, routing_key) = rabbitmq::setup_rabbitmq_client().await;

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
