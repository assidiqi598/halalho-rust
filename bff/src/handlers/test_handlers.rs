use std::sync::Arc;

use axum::{Json, extract::State};
use serde_json::json;

use crate::{
    dtos::general_res_dto::GeneralResDto,
    types::{app_state::AppState, error::CustomError},
};

pub async fn test_publish(
    State(state): State<Arc<AppState>>,
) -> Result<Json<GeneralResDto>, CustomError> {
    let msg = json!({
        "user_id": "a1b2c3d4e5f6g7h8i9j0".to_owned(),
    });

    let msg = serde_json::to_vec(&msg).map_err(|_| CustomError::SerializationError)?;

    state
        .rabbitmq_service
        .publish("amq.topic", "users.reg.email", &msg)
        .await?;

    Ok(Json(GeneralResDto {
        status_code: 200,
        message: "Sent".to_owned(),
    }))
}
