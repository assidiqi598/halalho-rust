use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct GeneralResDto {
    pub status_code: u16,
    pub message: String
}