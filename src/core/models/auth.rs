use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub struct Auth {
    id: i64,
    user_id: i64,
    refresh_token: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    pub token: String,
    pub user_id: i64,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidPayload,
    InvalidCredentials,
    TokenExpired,
    InternalError,
    EmailAlreadyExists
}