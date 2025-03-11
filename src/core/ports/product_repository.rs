use crate::core::models::{
    product::{Product, ProductError},
    product_category::{ProductCategory, ProductCategoryError},
};

pub trait ProductRepository: Send + Sync {
    fn create_product(
        &mut self,
        name: String,
        description: String,
        price: f64,
        stock: i32,
        product_image: Option<String>,
    ) -> Result<Product, ProductError>;

    fn find_product_by_id(&mut self, id: i64) -> Result<Product, ProductError>;

    fn find_all_products(&mut self) -> Result<Vec<Product>, ProductError>;

    fn find_products_by_name(&mut self, name: String) -> Result<Vec<Product>, ProductError>;

    fn delete_product(&mut self, id: i64) -> Result<(), ProductError>;

    fn update_product(
        &mut self,
        id: i64,
        new_name: String,
        new_description: String,
        new_price: f64,
        new_stock: i32,
    ) -> Result<Product, ProductError>;

    fn update_product_stock(&mut self, id: i64, new_stock: i32) -> Result<Product, ProductError>;
}

pub trait ProductCategoryRepository {
    fn create_product_category(
        &self,
        product_category: ProductCategory,
    ) -> Result<ProductCategory, ProductCategoryError>;

    fn find_product_category_by_id(&self, id: i64)
    -> Result<ProductCategory, ProductCategoryError>;

    fn find_categories_by_product_id(
        &self,
        product_id: i64,
    ) -> Result<Vec<ProductCategory>, ProductCategoryError>;

    fn find_products_by_category_id(
        &self,
        category_id: i64,
    ) -> Result<Vec<ProductCategory>, ProductCategoryError>;

    fn delete_product_category(&self, id: i64) -> Result<(), ProductCategoryError>;

    fn delete_product_categories_by_product_id(
        &self,
        product_id: i64,
    ) -> Result<(), ProductCategoryError>;

    fn delete_product_categories_by_category_id(
        &self,
        category_id: i64,
    ) -> Result<(), ProductCategoryError>;
}
