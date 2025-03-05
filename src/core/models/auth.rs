use chrono::NaiveDateTime;

pub struct Auth {
    id: i64,
    user_id: i64,
    refresh_token: String,
    pub created_at: NaiveDateTime,
}

pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

pub struct RegisterCredentials {
    pub email: String,
    pub password: String,
}

pub struct RefreshToken {
    pub token: String,
    pub user_id: i64,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug)]
pub enum AuthError {
    UserNotFound,
    InvalidCredentials,
    EmailAlreadyExists,
    TokenExpired,
    InternalError,
}
