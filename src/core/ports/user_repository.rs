use crate::core::models::{
    auth::{AuthError, LoginCredentials, RegisterCredentials},
    user::User,
};

pub trait UserRepository {
    fn register(&mut self, credentials: RegisterCredentials) -> Result<User, AuthError>;
    fn login(&mut self, credentials: LoginCredentials) -> Result<User, AuthError>;
    fn find_by_email(&mut self, email: &str) -> Result<User, AuthError>;
}
