use diesel::{PgConnection, RunQueryDsl};

use crate::{
    adapters::postgres::entities::{NewProductEntity, ProductEntity},
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

        let query = diesel::insert_into(crate::adapters::postgres::schema::products::table)
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
            });

        query.map_err(Into::into)
    }
}
