use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    pub user_id: i64,
}

#[derive(Debug, Display)]
pub enum AuthError {
    InternalError,
    InvalidPayload,
    InvalidCredentials,
    TokenExpired,
    EmailAlreadyExists,
    EmailNotVerified,
}