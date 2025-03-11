use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, query_dsl::methods::FilterDsl};

use crate::{
    adapters::postgres::{
        entities::user::{NewUserEntity, UserEntity},
        schema::users,
    },
    core::{
        models::user::{User, UserError, UserRole},
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
        first_name: String,
        last_name: String,
        email: String,
        password_hash: String,
    ) -> Result<User, UserError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        diesel::insert_into(users::table)
            .values(NewUserEntity {
                first_name,
                last_name,
                email,
                password_hash,
            })
            .get_result::<UserEntity>(conn_borrow.deref_mut())
            .map(|entity| entity.to_model())
            .map_err(|err| match err {
                diesel::result::Error::DatabaseError(kind, _) => match kind {
                    diesel::result::DatabaseErrorKind::UniqueViolation => {
                        UserError::EmailAlreadyExists
                    }
                    _ => UserError::InternalError,
                },
                _ => UserError::InternalError,
            })
    }

    fn find_by_email(&mut self, email: &str) -> Result<User, UserError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        users::table
            .filter(users::email.eq(email))
            .first::<UserEntity>(conn_borrow.deref_mut())
            .map(|entity| entity.to_model())
            .map_err(|_| UserError::UserNotFound)
    }

    fn find_by_id(&mut self, id: i64) -> Result<User, UserError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        users::table
            .filter(users::id.eq(id))
            .first::<UserEntity>(conn_borrow.deref_mut())
            .map(|entity| entity.to_model())
            .map_err(|_| UserError::UserNotFound)
    }
    
    fn has_role(&mut self, user_id: i64, roles: Vec<UserRole>) -> bool {
        match self.find_by_id(user_id) {
            Ok(user) => roles.contains(&user.user_role),
            Err(_) => false,
        }
    }
}
