use diesel::{pg::Pg, prelude::*};
use chrono::NaiveDateTime;
use crate::adapters::postgres::schema::*;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct UserEntity {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUserEntity {
    pub email: String,
    pub password_hash: String,
}