use chrono::NaiveDateTime;
use derive_more::Display;
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

#[derive(Debug, Display)]
pub enum AuthError {
    InternalError,
    InvalidPayload,
    InvalidCredentials,
    TokenExpired,
    EmailAlreadyExists
}