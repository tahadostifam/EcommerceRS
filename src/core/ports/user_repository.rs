use crate::core::models::{user::UserError, user::User};

pub trait UserRepository {
    fn create(&mut self, name: String, email: String, password_hash: String) -> Result<User, UserError>;
    fn find_by_email(&mut self, email: &str) -> Result<User, UserError>;
    fn find_by_id(&mut self, id: i64) -> Result<User, UserError>;
}
