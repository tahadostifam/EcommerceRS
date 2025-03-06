use diesel::{
    ExpressionMethods, PgConnection, RunQueryDsl, TextExpressionMethods,
    query_dsl::methods::FilterDsl,
};

use crate::{
    adapters::postgres::{
        entities::{NewProductEntity, ProductEntity},
        schema::products,
    },
    core::{
        models::product::{Product, ProductError},
        ports::product_repository::ProductRepository,
    },
};

pub struct ProductRepositoryImpl {
    pub conn: PgConnection,
}

impl ProductRepositoryImpl {
    pub fn new(conn: PgConnection) -> Self {
        ProductRepositoryImpl { conn }
    }
}

impl ProductRepository for ProductRepositoryImpl {
    fn create_product(
        &mut self,
        name: String,
        description: String,
        price: f64,
        stock: i32,
    ) -> Result<Product, ProductError> {
        let entity = NewProductEntity {
            name,
            description,
            price,
            stock,
        };

        diesel::insert_into(products::table)
            .values(&entity)
            .get_result::<ProductEntity>(&mut self.conn)
            .map(|entity| Product {
                id: entity.id,
                name: entity.name,
                description: entity.description,
                price: entity.price,
                stock: entity.stock,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(Into::into)
    }

    fn find_product_by_id(&mut self, id: i64) -> Result<Product, ProductError> {
        products::table
            .filter(products::id.eq(id))
            .first::<ProductEntity>(&mut self.conn)
            .map(|entity| Product {
                id: entity.id,
                name: entity.name,
                description: entity.description,
                price: entity.price,
                stock: entity.stock,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(Into::into)
    }

    fn find_all_products(&mut self) -> Result<Vec<Product>, ProductError> {
        products::table
            .load::<ProductEntity>(&mut self.conn)
            .map(|entities| {
                entities
                    .into_iter()
                    .map(|entity| Product {
                        id: entity.id,
                        name: entity.name,
                        description: entity.description,
                        price: entity.price,
                        stock: entity.stock,
                        created_at: entity.created_at,
                        updated_at: entity.updated_at,
                    })
                    .collect()
            })
            .map_err(Into::into)
    }

    fn delete_product(&mut self, id: i64) -> Result<(), ProductError> {
        diesel::delete(products::table.filter(products::id.eq(id)))
            .execute(&mut self.conn)
            .map(|affected_rows| {
                if affected_rows == 0 {
                    Err(ProductError::NotFound)
                } else {
                    Ok(())
                }
            })
            .map_err(|_| ProductError::DatabaseError)?
    }

    fn find_products_by_name(&mut self, name: String) -> Result<Vec<Product>, ProductError> {
        products::table
            .filter(products::name.like(format!("%{}%", name))) // Using LIKE for substring search
            .load::<ProductEntity>(&mut self.conn)
            .map(|entities| {
                entities
                    .into_iter()
                    .map(|entity| Product {
                        id: entity.id,
                        name: entity.name,
                        description: entity.description,
                        price: entity.price,
                        stock: entity.stock,
                        created_at: entity.created_at,
                        updated_at: entity.updated_at,
                    })
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
    ) -> Result<Product, ProductError> {
        diesel::update(products::table.filter(products::id.eq(id)))
            .set((
                products::name.eq(new_name),
                products::description.eq(new_description),
                products::price.eq(new_price),
                products::stock.eq(new_stock),
            ))
            .get_result::<ProductEntity>(&mut self.conn)
            .map(|entity| Product {
                id: entity.id,
                name: entity.name,
                description: entity.description,
                price: entity.price,
                stock: entity.stock,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(Into::into)
    }

    fn update_product_stock(&mut self, id: i64, new_stock: i32) -> Result<Product, ProductError> {
        diesel::update(products::table.filter(products::id.eq(id)))
            .set(products::stock.eq(new_stock))
            .get_result::<ProductEntity>(&mut self.conn)
            .map(|entity| Product {
                id: entity.id,
                name: entity.name,
                description: entity.description,
                price: entity.price,
                stock: entity.stock,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(Into::into)
    }
}
