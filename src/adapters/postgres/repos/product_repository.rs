use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use diesel::{
    ExpressionMethods, PgConnection, RunQueryDsl, TextExpressionMethods,
    query_dsl::methods::FilterDsl,
};

use crate::{
    adapters::postgres::{
        entities::product::{NewProductEntity, ProductEntity},
        schema::products,
    },
    core::{
        models::{
            category::Category,
            product::{Product, ProductError},
        },
        ports::{category_repository::CategoryRepository, product_repository::ProductRepository},
    },
};

pub struct ProductRepositoryImpl {
    conn: Arc<Mutex<PgConnection>>,
    category_repo: Arc<Mutex<dyn CategoryRepository>>,
}

impl ProductRepositoryImpl {
    pub fn new(
        conn: Arc<Mutex<PgConnection>>,
        category_repo: Arc<Mutex<dyn CategoryRepository>>,
    ) -> Self {
        ProductRepositoryImpl {
            conn,
            category_repo,
        }
    }
}

impl ProductRepository for ProductRepositoryImpl {
    fn create_product(
        &mut self,
        name: String,
        description: String,
        price: f64,
        stock: i32,
        product_image: Option<String>,
        category_id: Option<i64>,
    ) -> Result<Product, ProductError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        let entity = NewProductEntity {
            name,
            description,
            price,
            stock,
            product_image,
            category_id,
        };

        let mut category: Option<Category> = None;
        if let Some(category_id) = category_id {
            let mut category_repo = self.category_repo.lock().unwrap();
            category = Some(
                category_repo
                    .find_category_by_id(category_id)
                    .map_err(|_| ProductError::InvalidCategory)?,
            );
        }

        diesel::insert_into(products::table)
            .values(&entity)
            .get_result::<ProductEntity>(conn_borrow.deref_mut())
            .map(|entity| entity.to_model(category))
            .map_err(Into::into)
    }

    fn find_product_by_id(&mut self, id: i64) -> Result<Product, ProductError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        let record = products::table
            .filter(products::id.eq(id))
            .first::<ProductEntity>(conn_borrow.deref_mut())
            .map_err(ProductError::from)?;

        let mut category: Option<Category> = None;
        if let Some(category_id) = record.category_id {
            let mut category_repo = self.category_repo.lock().unwrap();
            category = Some(
                category_repo
                    .find_category_by_id(category_id)
                    .map_err(|_| ProductError::InvalidCategory)?,
            );
        }

        Ok(record.to_model(category))
    }

    fn find_all_products(&mut self) -> Result<Vec<Product>, ProductError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        products::table
            .load::<ProductEntity>(conn_borrow.deref_mut())
            .map(|entities| {
                entities
                    .into_iter()
                    .map(|entity| entity.to_model(None))
                    .collect()
            })
            .map_err(Into::into)
    }

    fn delete_product(&mut self, id: i64) -> Result<(), ProductError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        diesel::delete(products::table.filter(products::id.eq(id)))
            .execute(conn_borrow.deref_mut())
            .map(|affected_rows| {
                if affected_rows == 0 {
                    Err(ProductError::NotFound)
                } else {
                    Ok(())
                }
            })
            .map_err(|_| ProductError::InternalError)?
    }

    fn find_products_by_name(&mut self, name: String) -> Result<Vec<Product>, ProductError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        products::table
            .filter(products::name.like(format!("%{}%", name))) // Using LIKE for substring search
            .load::<ProductEntity>(conn_borrow.deref_mut())
            .map(|entities| {
                entities
                    .into_iter()
                    .map(|entity| entity.to_model(None))
                    .collect()
            })
            .map_err(Into::into)
    }

    fn update_product(
        &mut self,
        id: i64,
        new_name: String,
        new_description: String,
        new_price: f64,
        new_stock: i32,
        new_product_image: Option<String>,
        new_category_id: Option<i64>,
    ) -> Result<Product, ProductError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        let record = diesel::update(products::table.filter(products::id.eq(id)))
            .set((
                products::name.eq(new_name),
                products::description.eq(new_description),
                products::price.eq(new_price),
                products::stock.eq(new_stock),
                products::product_image.eq(new_product_image),
                products::category_id.eq(new_category_id),
            ))
            .get_result::<ProductEntity>(conn_borrow.deref_mut())
            .map_err(ProductError::from)?;

        let mut category: Option<Category> = None;
        if let Some(category_id) = record.category_id {
            let mut category_repo = self.category_repo.lock().unwrap();
            category = Some(
                category_repo
                    .find_category_by_id(category_id)
                    .map_err(|_| ProductError::InvalidCategory)?,
            );
        }

        Ok(record.to_model(category))
    }
}
