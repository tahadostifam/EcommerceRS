use crate::adapters::postgres::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct UserEntity {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUserEntity {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}
