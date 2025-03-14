use crate::adapters::postgres::entities::category::{CategoryEntity, NewCategoryEntity};
use crate::core::ports::category_repository::CategoryRepository;
use crate::{
    adapters::postgres::schema::categories,
    core::models::category::{Category, CategoryError},
};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use std::ops::DerefMut;
pub struct CategoryRepositoryImpl {
    conn: Pool<ConnectionManager<PgConnection>>,
}

impl CategoryRepositoryImpl {
    pub fn new(conn: Pool<ConnectionManager<PgConnection>>) -> Self {
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
        let mut parent: Option<Box<Category>> = None;
        if let Some(parent_id) = parent_id {
            let records = self.find_all_categories()?;
            let record = records.iter().find(|&key| key.id == parent_id);
            if let Some(record) = record {
                parent = Some(Box::new(record.clone()));
            }
        }

        let mut conn = self.conn.get().unwrap();
        Ok(diesel::insert_into(categories::table)
            .values(NewCategoryEntity {
                name,
                description,
                parent_id,
            })
            .get_result::<CategoryEntity>(conn.deref_mut())
            .map(|entity| entity.to_model(parent))
            .map_err(|_| CategoryError::InternalError)?)
    }

    fn find_category_by_id(&mut self, id: i64) -> Result<Category, CategoryError> {
        let records: Vec<Category> = self.find_all_categories()?;

        if let Some(category) = records.iter().find(|&key| key.id == id) {
            Ok(category.clone())
        } else {
            Err(CategoryError::NotFound)
        }
    }

    fn find_all_categories(&mut self) -> Result<Vec<Category>, CategoryError> {
        let mut conn = self.conn.get().unwrap();

        let records = categories::table
            .load::<CategoryEntity>(conn.deref_mut())
            .map_err(|_| CategoryError::InternalError)?;

        let categories: Vec<Category> = records
            .clone()
            .into_iter()
            .map(|entity| {
                let mut parent: Option<Box<Category>> = None;
                if let Some(parent_id) = entity.parent_id.clone() {
                    let record = records.iter().find(|&key| key.id == parent_id);
                    if let Some(record) = record {
                        parent = Some(Box::new(record.to_model(None)));
                    }
                }

                entity.to_model(parent)
            })
            .collect();

        Ok(categories)
    }

    fn update_category(
        &mut self,
        category_id: i64,
        new_name: String,
        new_description: String,
        new_parent_id: Option<i64>,
    ) -> Result<Category, CategoryError> {
        let record = {
            let mut conn = self.conn.get().unwrap();
            diesel::update(categories::table.filter(categories::id.eq(category_id)))
                .set((
                    categories::name.eq(new_name),
                    categories::description.eq(new_description),
                    categories::parent_id.eq(new_parent_id),
                ))
                .get_result::<CategoryEntity>(conn.deref_mut())
                .map_err(|_| CategoryError::NotFound)?
        };

        self.find_category_by_id(record.id)
    }

    fn delete_category(&mut self, id: i64) -> Result<(), CategoryError> {
        let mut conn = self.conn.get().unwrap();

        diesel::delete(categories::table.filter(categories::id.eq(id)))
            .execute(conn.deref_mut())
            .map(|affected_rows| {
                if affected_rows == 0 {
                    Err(CategoryError::NotFound)
                } else {
                    Ok(())
                }
            })
            .map_err(|_| CategoryError::InternalError)?
    }
}
