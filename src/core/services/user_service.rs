use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    config::Config,
    core::{
        models::{
            auth::AuthError,
            user::{User, UserError},
        },
        ports::{auth_repository::AuthRepository, user_repository::UserRepository},
    },
};
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use chrono::{Duration, NaiveDateTime, TimeDelta, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

use super::email_service::EmailService;

const REFRESH_TOKEN_LENGTH: i32 = 30;
const REFRESH_TOKEN_TTL: TimeDelta = Duration::days(30);

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub user_id: i64,
}

#[derive(Clone)]
pub struct UserService {
    pub(crate) jwt_secret: String,
    pub(crate) auth_repo: Arc<Mutex<dyn AuthRepository>>,
    pub(crate) user_repo: Arc<Mutex<dyn UserRepository>>,
    pub(crate) email_service: Arc<Mutex<dyn EmailService>>,
}

pub fn new_user_service(
    jwt_secret: String,
    auth_repo: Arc<Mutex<dyn AuthRepository>>,
    user_repo: Arc<Mutex<dyn UserRepository>>,
    email_service: Arc<Mutex<dyn EmailService>>,
) -> UserService {
    UserService {
        jwt_secret,
        auth_repo,
        user_repo,
        email_service,
    }
}

impl UserService {
    pub fn register(
        &mut self,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
    ) -> Result<User, AuthError> {
        let mut user_repo = self.user_repo.lock().unwrap();
        let email_service = self.email_service.lock().unwrap();
        let password_hash = self.hash_password(password)?;

        let user = user_repo
            .create(first_name, last_name, email, password_hash)
            .map_err(|err| {
                dbg!(err.clone());
                match err {
                    UserError::EmailAlreadyExists => AuthError::EmailAlreadyExists,
                    _ => AuthError::InternalError,
                }
            })?;

        let email_context: HashMap<String, String> = HashMap::new();
        email_service
            .send_email(
                &user.email,
                "Verify Your Email",
                "verify_email",
                email_context,
            )
            .map_err(|_| AuthError::InternalError)?;

        Ok(user)
    }

    pub fn login(
        &mut self,
        email: String,
        password: String,
    ) -> Result<(User, String, String), AuthError> {
        let mut user_repo = self.user_repo.lock().unwrap();
        let mut auth_repo = self.auth_repo.lock().unwrap();

        let user = user_repo
            .find_by_email(&email)
            .map_err(|_| AuthError::InvalidCredentials)?;

        if !self.verify_password(password, user.password_hash.clone()) {
            return Err(AuthError::InvalidCredentials);
        }

        if !user.email_verified {
            return Err(AuthError::EmailNotVerified);
        }

        let refresh_token = self.generate_refresh_token();
        auth_repo
            .save_refresh_token(
                user.id,
                refresh_token.clone(),
                self.refresh_token_expires_at(),
            )
            .map_err(|_| AuthError::InternalError)?;

        let access_token = self
            .generate_access_token(user.id)
            .map_err(|_| AuthError::InternalError)?;

        Ok((user, refresh_token, access_token))
    }

    fn generate_access_token(&self, user_id: i64) -> jsonwebtoken::errors::Result<String> {
        let claims = AccessTokenClaims { user_id };

        let header = Header::default();
        let encoding_key = EncodingKey::from_secret(self.jwt_secret.as_bytes());
        jsonwebtoken::encode(&header, &claims, &encoding_key)
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

    fn verify_password(&self, password: String, hashed_password: String) -> bool {
        match PasswordHash::new(&hashed_password) {
            Ok(parsed_hash) => {
                match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }
}
