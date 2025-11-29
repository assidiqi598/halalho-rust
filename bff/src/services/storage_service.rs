use aws_sdk_s3::Client;
use bytes::Bytes;

use crate::config::r2::BUCKET;

pub struct StorageService {
    pub r2_client: Client,
}

impl StorageService {
    pub fn new(client: Client) -> Self {
        Self { r2_client: client }
    }

    pub async fn get_object(&self, key: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
        let resp = self
            .r2_client
            .get_object()
            .bucket(BUCKET.clone())
            .key(key)
            .send()
            .await?;

        let data = resp.body.collect().await?;
        Ok(data.into_bytes())
    }
}
