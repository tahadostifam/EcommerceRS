use std::sync::{Arc, Mutex};

use crate::core::{
    models::product::{Product, ProductError},
    ports::product_repository::ProductRepository,
};
#[derive(Clone)]
pub struct ProductService {
    pub(crate) product_repo: Arc<Mutex<dyn ProductRepository>>,
}

pub fn new_product_service(product_repo: Arc<Mutex<dyn ProductRepository>>) -> ProductService {
    ProductService { product_repo }
}

impl ProductService {
    pub fn get(&mut self, product_id: i64) -> Result<Product, ProductError> {
        let mut product_repo = self.product_repo.lock().unwrap();
        let product = product_repo.find_product_by_id(product_id)?;
        Ok(product)
    }

    pub fn search(&mut self, search: String) -> Result<Vec<Product>, ProductError> {
        let mut product_repo: std::sync::MutexGuard<'_, dyn ProductRepository> =
            self.product_repo.lock().unwrap();
        let product = product_repo.find_products_by_name(search)?;
        Ok(product)
    }

    pub fn create(
        &mut self,
        name: String,
        description: String,
        price: f64,
        stock: i32,
        product_image: Option<String>,
    ) -> Result<Product, ProductError> {
        let mut product_repo = self.product_repo.lock().unwrap();

        let product =
            product_repo.create_product(name, description, price, stock, product_image)?;
        Ok(product)
    }

    pub fn update(
        &mut self,
        id: i64,
        new_name: String,
        new_description: String,
        new_price: f64,
        new_stock: i32,
        new_product_image: Option<String>,
    ) -> Result<Product, ProductError> {
        let mut product_repo: std::sync::MutexGuard<'_, dyn ProductRepository> =
            self.product_repo.lock().unwrap();

        let product = product_repo.update_product(
            id,
            new_name,
            new_description,
            new_price,
            new_stock,
            new_product_image,
        )?;
        Ok(product)
    }
}
