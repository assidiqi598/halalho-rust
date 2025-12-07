use crate::services::{
    auth_service::AuthService, email_service::EmailService,
    refresh_token_service::RefreshTokenService, storage_service::StorageService,
    user_service::UserService, verif_email_token_service::VerifEmailTokenService,
};

pub struct AppState {
    pub user_service: UserService,
    pub refresh_token_service: RefreshTokenService,
    pub auth_service: AuthService,
    pub storage_service: StorageService,
    pub email_service: EmailService,
    pub verif_email_token_service: VerifEmailTokenService,
}
