use aws_sdk_s3::Client;

use crate::config::r2::BUCKET;

pub struct StorageService {
    pub r2_client: Client,
}

impl StorageService {
    pub fn new(client: Client) -> Self {
        Self { r2_client: client }
    }

    pub async fn get_object(&self, key: &str) -> Result<(Vec<u8>, Option<String>), Box<dyn std::error::Error>> {
        let resp = self
            .r2_client
            .get_object()
            .bucket(BUCKET.clone())
            .key(key)
            .send()
            .await?;

        let content_type = resp.content_type().map(|s| s.to_string());
        let data = resp.body.collect().await?;

        Ok((data.to_vec(), content_type))
    }
}
