use crate::services::{
    auth_service::AuthService, email_service::EmailService,
    email_verif_token_service::VerifEmailTokenService, rabbitmq_service::RabbitmqService,
    refresh_token_service::RefreshTokenService, storage_service::StorageService,
    user_service::UserService,
};

pub struct AppState {
    pub auth_service: AuthService,
    pub email_service: EmailService,
    pub rabbitmq_service: RabbitmqService,
    pub refresh_token_service: RefreshTokenService,
    pub storage_service: StorageService,
    pub user_service: UserService,
    pub verif_email_token_service: VerifEmailTokenService,
}
