use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    dtos::general_res_dto::GeneralResDto,
    types::{app_state::AppState, error::CustomError},
};

pub async fn test_publish(
    State(state): State<Arc<AppState>>,
) -> Result<Json<GeneralResDto>, CustomError> {
    let msg = br#"{"type":"test", "event":"new"}"#;

    state
        .rabbitmq_service
        .publish("amq.topic", "users.reg.email", msg)
        .await?;

    Ok(Json(GeneralResDto {
        status_code: 200,
        message: "Sent".to_owned(),
    }))
}
