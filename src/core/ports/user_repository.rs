use crate::core::models::user::{User, UserError, UserRole};

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
    fn has_role(&mut self, user_id: i64, roles: Vec<UserRole>) -> bool;
}
