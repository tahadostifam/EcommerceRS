use std::{cell::RefCell, rc::Rc};

use crate::core::{
    models::{auth::AuthError, user::User},
    ports::{auth_repository::AuthRepository, user_repository::UserRepository},
};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use chrono::{Duration, NaiveDateTime, TimeDelta, Utc};

const REFRESH_TOKEN_LENGTH: i32 = 30;
const REFRESH_TOKEN_TTL: TimeDelta = Duration::days(30);

pub struct UserService {
    pub auth_repo: Rc<RefCell<dyn AuthRepository>>,
    pub user_repo: Rc<RefCell<dyn UserRepository>>,
}

pub fn new_user_service(
    auth_repo: Rc<RefCell<dyn AuthRepository>>,
    user_repo: Rc<RefCell<dyn UserRepository>>,
) -> UserService {
    UserService {
        auth_repo,
        user_repo,
    }
}

impl UserService {
    pub fn register(
        &mut self,
        name: String,
        email: String,
        password: String,
    ) -> Result<User, AuthError> {
        let mut user_repo = self.user_repo.borrow_mut();
        let password_hash = self.hash_password(password)?;

        let user = user_repo
            .create(name, email, password_hash)
            .map_err(|_| AuthError::EmailAlreadyExists)?;

        // TODO Send Verification Email

        Ok(user)
    }

    pub fn login(&mut self, email: String, password: String) -> Result<User, AuthError> {
        unimplemented!();
    }

    fn refresh_token_expires_at(&self) -> NaiveDateTime {
        Utc::now().naive_utc() + REFRESH_TOKEN_TTL
    }

    fn generate_refresh_token(&self) -> String {
        let mut rng = rand::thread_rng();
        (0..REFRESH_TOKEN_LENGTH)
            .map(|_| format!("{:02x}", rand::Rng::r#gen::<u8>(&mut rng)))
            .collect::<String>()
    }

    fn hash_password(&self, password: String) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AuthError::InternalError)?
            .to_string())
    }
}
