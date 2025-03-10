use crate::{adapters::postgres::schema::*, core::models::user::User};
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct UserEntity {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub profile_picture: Option<String>,
    pub password_hash: String,
    pub email_verified: bool,
    pub last_login: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserEntity {
    pub fn to_model(&self) -> User {
        User {
            id: self.id,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            email_verified: self.email_verified,
            profile_picture: self.profile_picture.clone(),
            password_hash: self.password_hash.clone(),
            last_login: self.last_login,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUserEntity {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
}
