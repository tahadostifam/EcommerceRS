use chrono::NaiveDateTime;

use crate::core::models::auth::{AuthError, RefreshToken};

pub trait AuthRepository {
    fn save_refresh_token(
        &mut self,
        user_id: i64,
        token: String,
        expires_at: NaiveDateTime,
    ) -> Result<(), AuthError>;
    fn validate_refresh_token(&mut self, user_id: i64, token: &str) -> Result<RefreshToken, AuthError>;
    fn delete_refresh_token(&mut self, user_id: i64, token: &str) -> Result<(), AuthError>;
}
