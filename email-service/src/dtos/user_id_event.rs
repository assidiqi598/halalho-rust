use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserIdEvent {
    pub user_id: String,
    pub email: String,
    pub username: String,
}