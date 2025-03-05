use chrono::NaiveDateTime;

use crate::core::models::{
    auth::{AuthError, LoginCredentials, RegisterCredentials},
    user::User,
};

pub trait UserRepository {
    fn register(&self, credentials: RegisterCredentials) -> Result<User, AuthError>;
    fn login(&self, credentials: LoginCredentials) -> Result<User, AuthError>;
    fn find_by_email(&self, email: &str) -> Result<User, AuthError>;
}
