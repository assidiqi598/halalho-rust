use amqprs::{
    BasicProperties,
    channel::{BasicPublishArguments, Channel},
    connection::Connection,
};

use crate::types::error::CustomError;

pub struct RabbitmqService {
    _connection: Connection,
    channel: Channel,
}

impl RabbitmqService {
    pub fn new(connection: Connection, channel: Channel) -> Self {
        Self {
            _connection: connection,
            channel,
        }
    }

    pub async fn publish(
        &self,
        exchange: &str,
        routing_key: &str,
        body: &[u8],
    ) -> Result<(), CustomError> {
        let args = BasicPublishArguments::new(exchange, routing_key);

        self.channel
            .basic_publish(BasicProperties::default(), body.to_vec(), args)
            .await?;

        Ok(())
    }
}
