use std::{ops::DerefMut, sync::{Arc, Mutex}};

use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, query_dsl::methods::FilterDsl};

use crate::{
    adapters::postgres::{
        entities::user::{NewUserEntity, UserEntity},
        schema::users,
    },
    core::{
        models::user::{User, UserError},
        ports::user_repository::UserRepository,
    },
};

pub struct UserRepositoryImpl {
    conn: Arc<Mutex<PgConnection>>,
}

impl UserRepositoryImpl {
    pub fn new(conn: Arc<Mutex<PgConnection>>) -> Self {
        UserRepositoryImpl { conn }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn create(
        &mut self,
        name: String,
        email: String,
        password_hash: String,
    ) -> Result<User, UserError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        diesel::insert_into(users::table)
            .values(NewUserEntity {
                name,
                email,
                password_hash,
            })
            .get_result::<UserEntity>(conn_borrow.deref_mut())
            .map(|entity| User {
                id: entity.id,
                name: entity.name,
                email: entity.email,
                password_hash: entity.password_hash,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| UserError::EmailAlreadyExists)
    }

    fn find_by_email(&mut self, email: &str) -> Result<User, UserError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        users::table
            .filter(users::email.eq(email))
            .first::<UserEntity>(conn_borrow.deref_mut())
            .map(|entity| User {
                id: entity.id,
                name: entity.name,
                email: entity.email,
                password_hash: entity.password_hash,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| UserError::UserNotFound)
    }

    fn find_by_id(&mut self, id: i64) -> Result<User, UserError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        users::table
            .filter(users::id.eq(id))
            .first::<UserEntity>(conn_borrow.deref_mut())
            .map(|entity| User {
                id: entity.id,
                name: entity.name,
                email: entity.email,
                password_hash: entity.password_hash,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| UserError::UserNotFound)
    }
}
