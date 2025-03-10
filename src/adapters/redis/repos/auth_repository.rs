use crate::core::{
    models::auth::{AuthError, RefreshToken},
    ports::auth_repository::AuthRepository,
};
use redis::Commands;
use std::sync::{Arc, Mutex};

pub struct AuthRepositoryImpl {
    conn: Arc<Mutex<redis::Client>>,
}

impl AuthRepositoryImpl {
    pub fn new(conn: Arc<Mutex<redis::Client>>) -> Self {
        AuthRepositoryImpl { conn }
    }
}

impl AuthRepository for AuthRepositoryImpl {
    fn save_refresh_token(
        &mut self,
        user_id: i64,
        token: String,
        expires_at: chrono::NaiveDateTime,
    ) -> Result<(), AuthError> {
        let mut client = self
            .conn
            .lock()
            .unwrap()
            .get_connection()
            .map_err(|_| AuthError::InternalError)?;

        let value = serde_json::to_string(&RefreshToken { user_id })
            .map_err(|_| AuthError::InvalidPayload)?;

        Ok(client
            .set_ex(
                format!("user::{}::{}", user_id, token),
                value,
                expires_at.and_utc().timestamp() as u64,
            )
            .map_err(|_| AuthError::InternalError)?)
    }

    fn validate_refresh_token(&mut self, token: &str) -> Result<RefreshToken, AuthError> {
        Ok(self.find_refresh_token(token.to_string())?.1)
    }

    fn remove_refresh_token(&mut self, token: &str) -> Result<(), AuthError> {
        let mut client = self
            .conn
            .lock()
            .unwrap()
            .get_connection()
            .map_err(|_| AuthError::InternalError)?;

        let key = self.find_refresh_token(token.to_string())?.0;

        client
            .del::<String, String>(key)
            .map_err(|_| AuthError::InternalError)?;

        Ok(())
    }

    fn terminal_user_sessions(&mut self, user_id: &str) -> Result<(), AuthError> {
        let mut client = self
            .conn
            .lock()
            .unwrap()
            .get_connection()
            .map_err(|_| AuthError::InternalError)?;

        let pattern = format!("user::{}::*", user_id);
        let keys: Vec<String> = client
            .scan_match(pattern)
            .map_err(|_| AuthError::InternalError)?
            .collect();

        for key in keys {
            self.remove_refresh_token(&key)
                .map_err(|_| AuthError::InternalError)?;
        }

        Ok(())
    }
}

impl AuthRepositoryImpl {
    fn find_refresh_token(&mut self, token: String) -> Result<(String, RefreshToken), AuthError> {
        let mut client = self
            .conn
            .lock()
            .unwrap()
            .get_connection()
            .map_err(|_| AuthError::InternalError)?;

        let pattern = format!("user::*::{}", token);
        let keys: Vec<String> = client
            .scan_match(pattern)
            .map_err(|_| AuthError::InternalError)?
            .collect();

        if let Some(key) = keys.get(0) {
            let payload_str: String = client.get(key).map_err(|_| AuthError::InternalError)?;

            let payload: RefreshToken =
                serde_json::from_str(&payload_str).map_err(|_| AuthError::InvalidCredentials)?;

            return Ok((format!("user::{}::{}", payload.user_id, token), payload));
        }

        Err(AuthError::InvalidCredentials)
    }
}
