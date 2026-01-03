use amqprs::{
    BasicProperties, Deliver,
    channel::{BasicAckArguments, Channel},
    consumer::AsyncConsumer,
};

use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    dtos::user_id_event::UserIdEvent, handlers::send_email_verif_handler::send_email_verification,
    services::{email_service::EmailService, email_verif_token_service::VerifEmailTokenService, storage_service::StorageService},
};

pub struct MainServiceConsumer {
    pub verif_email_token_service: Arc<VerifEmailTokenService>,
    pub storage_service: Arc<StorageService>,
    pub email_service: Arc<EmailService>,
}

#[async_trait]
impl AsyncConsumer for MainServiceConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _props: BasicProperties,
        content: Vec<u8>,
    ) {
        let body = std::str::from_utf8(&content)
            .map_err(|_| {
                tracing::error!("Failed to parse message body as UTF-8 string");
                tracing::info!(delivery_tag = deliver.delivery_tag(), body = ?content);
            })
            .unwrap();

        tracing::info!(
            "Received message {} from routing key {} with delivery tag {}",
            body,
            deliver.routing_key(),
            deliver.delivery_tag()
        );

        match deliver.routing_key().as_str() {
            "users.reg.email" => {
                tracing::info!("Process user registration email for body");

                let user_id_event: UserIdEvent = match serde_json::from_str(body) {
                    Ok(event) => event,
                    Err(e) => {
                        tracing::error!("Failed to deserialize message body: {}", e);
                        return;
                    }
                };

                tracing::info!("User ID from event: {}", user_id_event.user_id);

                let _ = send_email_verification(
                    self.email_service.clone(),
                    self.verif_email_token_service.clone(),
                    self.storage_service.clone(),
                    user_id_event.user_id,
                    user_id_event.email,
                    user_id_event.username,
                ).await.map_err(|_err| tracing::error!("error when sending email verification")) ;
            }
            _ => {
                tracing::warn!(
                    "Unknown routing key: {}. Message body: {}",
                    deliver.routing_key(),
                    body
                );
            }
        }

        channel
            .basic_ack(BasicAckArguments::new(deliver.delivery_tag(), false))
            .await
            .unwrap();
    }
}
