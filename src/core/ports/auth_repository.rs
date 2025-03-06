use chrono::NaiveDateTime;

use crate::core::models::{auth::AuthError, user::User};

pub trait AuthRepository {
    fn save_refresh_token(
        &mut self,
        user_id: i64,
        token: &str,
        expires_at: NaiveDateTime,
    ) -> Result<(), AuthError>;
    fn validate_refresh_token(&mut self, token: &str) -> Result<User, AuthError>;
    fn delete_refresh_token(&mut self, token: &str) -> Result<(), AuthError>;
}
