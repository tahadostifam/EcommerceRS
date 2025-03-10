use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub profile_picture: Option<String>,

    #[serde(skip_serializing)]
    pub password_hash: String,

    pub email_verified: bool,
    pub last_login: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub enum UserError {
    InternalError,
    UserNotFound,
    EmailAlreadyExists,
}
