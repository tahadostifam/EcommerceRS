use std::{cell::RefCell, rc::Rc};

use redis::Commands;

use crate::core::{
    models::auth::{AuthError, RefreshToken},
    ports::auth_repository::AuthRepository,
};

pub struct AuthRepositoryImpl {
    conn: Rc<RefCell<redis::Client>>,
}

impl AuthRepositoryImpl {
    pub fn new(conn: Rc<RefCell<redis::Client>>) -> Self {
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
            .borrow_mut()
            .get_connection()
            .map_err(|_| AuthError::InternalError)?;

        let value = serde_json::to_string(&RefreshToken {
            token: token.clone(),
            user_id,
            expires_at,
        })
        .map_err(|_| AuthError::InvalidPayload)?;

        Ok(client
            .hset::<String, String, String, ()>(format!("user::{}", user_id), token, value)
            .map_err(|_| AuthError::InternalError)?)
    }

    fn validate_refresh_token(
        &mut self,
        user_id: i64,
        token: &str,
    ) -> Result<RefreshToken, AuthError> {
        let mut client = self
            .conn
            .borrow_mut()
            .get_connection()
            .map_err(|_| AuthError::InternalError)?;

        let refresh_token_str = client
            .hget::<String, String, String>(format!("user::{}", user_id), token.to_string())
            .map_err(|_| AuthError::InvalidCredentials)?;

        let refresh_token: RefreshToken =
            serde_json::from_str(&refresh_token_str).map_err(|_| AuthError::InvalidCredentials)?;

        Ok(refresh_token)
    }

    fn delete_refresh_token(&mut self, user_id: i64, token: &str) -> Result<(), AuthError> {
        let mut client = self
            .conn
            .borrow_mut()
            .get_connection()
            .map_err(|_| AuthError::InternalError)?;

        let _ = client
            .hdel::<String, String, String>(format!("user::{}", user_id), token.to_string())
            .map_err(|_| AuthError::InvalidCredentials);

        Ok(())
    }
}
