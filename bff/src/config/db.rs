use mongodb::error::Error;
use mongodb::{
    Client, Collection, Database, IndexModel,
    bson::doc,
    options::{ClientOptions, IndexOptions, ServerApi, ServerApiVersion},
};
use std::env;

use crate::models::token::Token;
use crate::models::user::User;

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

    let users_coll: Collection<User> = db.collection("users");
    let email_index_opts = IndexOptions::builder().unique(true).build();
    let email_index = IndexModel::builder()
        .keys(doc! { "email": 1})
        .options(email_index_opts)
        .build();
    let user_idx = users_coll.create_index(email_index).await?;
    println!("Created index:\n{}", user_idx.index_name);

    let tokens_coll: Collection<Token> = db.collection("tokens");
    let token_index_opts = IndexOptions::builder().unique(true).build();
    let token_index = IndexModel::builder()
        .keys(doc! { "token": 1})
        .options(token_index_opts)
        .build();
    let token_idx = tokens_coll.create_index(token_index).await?;
    println!("Created index:\n{}", token_idx.index_name);

    Ok(db)
}
