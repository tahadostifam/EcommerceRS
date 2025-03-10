use ecommercers::core::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserRegisterDTO {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserLoginDTO {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserLoggedInDTO {
    pub user: User,
    pub refresh_token: String,
    pub access_token: String,
}
