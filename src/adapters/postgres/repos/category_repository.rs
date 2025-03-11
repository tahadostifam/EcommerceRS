use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use diesel::prelude::*;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::adapters::postgres::entities::category::{CategoryEntity, NewCategoryEntity};
use crate::core::ports::category_repository::CategoryRepository;
use crate::{
    adapters::postgres::schema::categories,
    core::models::category::{Category, CategoryError},
};

pub struct CategoryRepositoryImpl {
    conn: Arc<Mutex<PgConnection>>,
}

impl CategoryRepositoryImpl {
    pub fn new(conn: Arc<Mutex<PgConnection>>) -> Self {
        CategoryRepositoryImpl { conn }
    }
}

impl CategoryRepository for CategoryRepositoryImpl {
    fn create_category(
        &mut self,
        name: String,
        description: String,
        parent_id: Option<i64>,
    ) -> Result<Category, CategoryError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        diesel::insert_into(categories::table)
            .values(NewCategoryEntity {
                name,
                description,
                parent_id,
            })
            .get_result::<CategoryEntity>(conn_borrow.deref_mut())
            .map(|entity| entity.to_model())
            .map_err(|_| CategoryError::InternalError)
    }

    fn find_category_by_id(&mut self, id: i64) -> Result<Category, CategoryError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        categories::table
            .filter(categories::id.eq(id))
            .first::<CategoryEntity>(conn_borrow.deref_mut())
            .map(|entity| entity.to_model())
            .map_err(|_| CategoryError::NotFound)
    }

    fn find_all_categories(&mut self) -> Result<Vec<Category>, CategoryError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        categories::table
            .load::<CategoryEntity>(conn_borrow.deref_mut())
            .map(|entities| {
                entities
                    .into_iter()
                    .map(|entity| entity.to_model())
                    .collect()
            })
            .map_err(|_| CategoryError::InternalError)
    }

    fn update_category(
        &mut self,
        category_id: i64,
        new_name: String,
        new_description: String,
        new_parent_id: Option<i64>,
    ) -> Result<Category, CategoryError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        diesel::update(categories::table.filter(categories::id.eq(category_id)))
            .set((
                categories::name.eq(new_name),
                categories::description.eq(new_description),
                categories::parent_id.eq(new_parent_id),
            ))
            .get_result::<CategoryEntity>(conn_borrow.deref_mut())
            .map(|entity| entity.to_model())
            .map_err(|_| CategoryError::NotFound)
    }

    fn delete_category(&mut self, id: i64) -> Result<(), CategoryError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        diesel::delete(categories::table.filter(categories::id.eq(id)))
            .execute(conn_borrow.deref_mut())
            .map(|affected_rows| {
                if affected_rows == 0 {
                    Err(CategoryError::NotFound)
                } else {
                    Ok(())
                }
            })
            .map_err(|_| CategoryError::InternalError)?
    }

    fn find_categories_by_name(&mut self, name: String) -> Result<Vec<Category>, CategoryError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        categories::table
            .filter(categories::name.like(format!("%{}%", name)))
            .load::<CategoryEntity>(conn_borrow.deref_mut())
            .map(|entities| {
                entities
                    .into_iter()
                    .map(|entity| entity.to_model())
                    .collect()
            })
            .map_err(|_| CategoryError::InternalError)
    }
}
