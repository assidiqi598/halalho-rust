use aws_sdk_s3 as s3;
use std::env::var;

pub async fn connect_r2() -> Result<s3::Client, s3::Error> {
    let bucket_name = var("R2_BUCKET_NAME").expect("R2_BUCKET_NAME is not set in env");
    let account_id = var("R2_ACCOUNT_ID").expect("R2_ACCOUNT_ID is not set in env");
    let access_key_id = var("R2_ACCESS_KEY_ID").expect("R2_ACCESS_KEY_ID is not set in env");
    let access_key_secret =
        var("R2_ACCESS_KEY_SECRET").expect("R2_ACCESS_KEY_SECRET is not set in env");

    let config = aws_config::from_env()
        .endpoint_url(format!("https://{}.r2.cloudflarestorage.com", account_id))
        .credentials_provider(s3::config::Credentials::new(
            access_key_id,
            access_key_secret,
            None,
            None,
            "R2",
        ))
        .region("auto")
        .load()
        .await;

    Ok(s3::Client::new(&config))
}
