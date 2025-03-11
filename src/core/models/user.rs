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
    pub user_role: UserRole,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Manager,
    Admin,
}

impl TryFrom<String> for UserRole {
    type Error = UserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "user" => UserRole::User,
            "manager" => UserRole::Manager,
            "admin" => UserRole::Admin,
            _ => return Err(UserError::InternalError),
        })
    }
}
