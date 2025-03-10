use chrono::NaiveDateTime;

use crate::core::models::auth::{AuthError, RefreshToken};

pub trait AuthRepository: Send + Sync {
    fn save_refresh_token(
        &mut self,
        user_id: i64,
        token: String,
        expires_at: NaiveDateTime,
    ) -> Result<(), AuthError>;
    fn validate_refresh_token(&mut self, token: &str) -> Result<RefreshToken, AuthError>;
    fn remove_refresh_token(&mut self, token: &str) -> Result<(), AuthError>;
    fn terminal_user_sessions(&mut self, user_id: &str) -> Result<(), AuthError>;
}
