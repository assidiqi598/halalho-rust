use mongodb::error::Error;
use mongodb::{
    Client, Collection, Database, IndexModel,
    bson::doc,
    options::{ClientOptions, IndexOptions, ServerApi, ServerApiVersion},
};
use std::env;
use std::time::Duration;

use crate::models::refresh_token::RefreshToken;
use crate::models::user::User;
use crate::services::auth_service::REFRESH_EXP_DAYS;

pub async fn connect_db() -> Result<Database, Error> {
    let uri = env::var("MONGO_URI").expect("MONGO_URI is not set in env");
    let mut client_options = ClientOptions::parse(uri).await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;
    // Send a ping to confirm a successful connection
    client
        .database("halalho")
        .run_command(doc! { "ping": 1 })
        .await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    let db = client.database("halalho");

    // email unique index for users
    let users_coll: Collection<User> = db.collection("users");
    let email_index_opts = IndexOptions::builder().unique(true).build();
    let email_index = IndexModel::builder()
        .keys(doc! { "email": 1})
        .options(email_index_opts)
        .build();
    let email_user_idx = users_coll.create_index(email_index).await?;
    tracing::info!(
        "Created unique index '{}' for users",
        email_user_idx.index_name
    );

    // username unique index for users
    let username_index_opts = IndexOptions::builder().unique(true).build();
    let username_index = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(username_index_opts)
        .build();
    let username_user_idx = users_coll.create_index(username_index).await?;
    tracing::info!(
        "Created unique index '{}' for users",
        username_user_idx.index_name
    );

    let tokens_coll: Collection<RefreshToken> = db.collection("refresh_tokens");
    let token_index_opts = IndexOptions::builder().unique(true).build();
    let token_index = IndexModel::builder()
        .keys(doc! { "token": 1})
        .options(token_index_opts)
        .build();
    let token_idx = tokens_coll.create_index(token_index).await?;
    tracing::info!("Created unique index '{}' for tokens", token_idx.index_name);

    let token_ttl_index_opts = IndexOptions::builder()
        .expire_after(Some(Duration::from_secs(REFRESH_EXP_DAYS as u64)))
        .build();
    let token_ttl_index = IndexModel::builder()
        .keys(doc! { "createdAt": 1})
        .options(token_ttl_index_opts)
        .build();
    let token_ttl_idx = tokens_coll.create_index(token_ttl_index).await?;
    tracing::info!(
        "Created ttl index '{}' for tokens",
        token_ttl_idx.index_name
    );

    Ok(db)
}
