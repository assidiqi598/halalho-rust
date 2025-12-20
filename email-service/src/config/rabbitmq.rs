use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicConsumeArguments, Channel, QueueBindArguments,
        QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    tls::TlsAdaptor,
};
use std::{env::var, path::PathBuf};

use crate::types::consumer::MainServiceConsumer;

pub async fn setup_rabbitmq_client() -> (Connection, Channel, String) {
    let cert_dir = PathBuf::from(var("RABBITMQ_CERT_DIR").expect("RABBITMQ_CERT_DIR missing"));

    let domain = var("RABBITMQ_CERT_DOMAIN").expect("RABBITMQ_CERT_DOMAIN missing");
    let root_ca_cert: PathBuf = cert_dir.join("ca_certificate.pem");
    let client_cert: PathBuf = cert_dir.join("client_certificate.pem");
    let client_key: PathBuf = cert_dir.join("client_key.pem");

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Must install crypto provider for tls.");

    let rabbitmq_domain = var("RABBITMQ_DOMAIN").expect("RABBITMQ_DOMAIN missing");
    let rabbitmq_port = var("RABBITMQ_PORT").expect("RABBITMQ_PORT missing");
    let rabbitmq_username = var("RABBITMQ_DEFAULT_USER").expect("RABBITMQ_DEFAULT_USER missing");
    let rabbitmq_pass = var("RABBITMQ_DEFAULT_PASS").expect("RABBITMQ_DEFAULT_PASS missing");

    let args = OpenConnectionArguments::new(
        &rabbitmq_domain,
        rabbitmq_port.parse::<u16>().unwrap(),
        &rabbitmq_username,
        &rabbitmq_pass,
    )
    .tls_adaptor(
        TlsAdaptor::with_client_auth(
            Some(root_ca_cert.as_path()),
            client_cert.as_path(),
            client_key.as_path(),
            domain.to_owned(),
        )
        .unwrap(),
    )
    .finish();

    let conn = Connection::open(&args).await.unwrap();
    conn.register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    let channel = conn.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    tracing::info!("Connected to rabbitmq");

    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::default())
        .await
        .unwrap()
        .unwrap();

    let routing_key = "*.*.email";
    let exchange_name = "amq.topic";
    channel
        .queue_bind(QueueBindArguments::new(
            &queue_name,
            exchange_name,
            routing_key,
        ))
        .await
        .unwrap();

    let args = BasicConsumeArguments::new(&queue_name, "email_consumer")
        .manual_ack(true)
        .finish();

    channel
        .basic_consume(MainServiceConsumer, args)
        .await
        .unwrap();

    (conn, channel, routing_key.to_owned())
}
