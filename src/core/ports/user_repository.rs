use crate::core::models::{user::User, user::UserError};

pub trait UserRepository: Send + Sync {
    fn create(
        &mut self,
        first_name: String,
        last_name: String,
        email: String,
        password_hash: String,
    ) -> Result<User, UserError>;
    fn find_by_email(&mut self, email: &str) -> Result<User, UserError>;
    fn find_by_id(&mut self, id: i64) -> Result<User, UserError>;
}
