use chrono::{TimeZone, Utc};
use std::sync::Arc;

use crate::{
    models::email_verif_token::NewEmailVerifToken,
    services::{
        email_service::{EmailService, EmailTemplateValues}, email_verif_token_service::VerifEmailTokenService, storage_service::StorageService,
    },
    types::{
        email::Email, error::CustomError, verify_email::{EMAIL_VERIFICATION_EXP_MINUTES, VerifyEmail}
    },
    utils::datetime::now_epoch,
};

pub async fn send_email_verification(
    email_service: Arc<EmailService>,
    verif_email_token_service: Arc<VerifEmailTokenService>,
    storage_service: Arc<StorageService>,
    user_id: String,
    email: String,
    username: String,
) -> Result<(), CustomError> {
    let (object_bytes, ext) = storage_service
        .get_object("halalho/email-templates/verify-email.html")
        .await
        .map_err(|_| CustomError::R2Error)?;

    let object_extension = ext.ok_or(CustomError::R2Error)?;

    let (raw_token, token_hash) = verif_email_token_service.generate_email_verification_token()?;

    let user_id = bson::oid::ObjectId::parse_str(&user_id).map_err(|e| {
        tracing::error!("Error while parsing {}: {:?}", user_id, e);
        CustomError::InvalidIDError(user_id.to_owned())
    })?;

    let new_verif_email_token = NewEmailVerifToken {
        userId: user_id,
        tokenHash: token_hash,
        expiresAt: Utc
            .timestamp_opt(
                (now_epoch() + EMAIL_VERIFICATION_EXP_MINUTES as usize) as i64,
                0,
            )
            .single()
            .ok_or_else(|| {
                tracing::error!("Error converting timestamp for verif email token expiration");
                CustomError::TokenCreation
            })?,
        createdAt: Utc::now(),
        usedAt: None,
    };

    verif_email_token_service
        .create_token(&new_verif_email_token)
        .await?;

    let values = EmailTemplateValues::VerifyEmailValues(VerifyEmail::new(
        &username,
        &user_id.to_hex(),
        &raw_token,
    ));

    let email_html = email_service.prepare_template(&object_bytes, &object_extension, values)?;

    let email: Email = Email::new(
        vec![(&username, &email)],
        email_html,
        "Please verify your email-address",
    );

    email_service.send_transactional_email(email).await?;

    Ok(())
}
