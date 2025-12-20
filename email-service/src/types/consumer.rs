use amqprs::{
    BasicProperties, Deliver,
    channel::{BasicAckArguments, Channel},
    consumer::AsyncConsumer,
};

use async_trait::async_trait;

pub struct MainServiceConsumer;

#[async_trait]
impl AsyncConsumer for MainServiceConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _props: BasicProperties,
        content: Vec<u8>,
    ) {
        match std::str::from_utf8(&content) {
            Ok(body) => tracing::info!(delivery_tag = deliver.delivery_tag(), body),
            Err(_) => tracing::info!(delivery_tag = deliver.delivery_tag(), body = ?content),
        }

        channel
            .basic_ack(BasicAckArguments::new(deliver.delivery_tag(), false))
            .await
            .unwrap();
    }
}
