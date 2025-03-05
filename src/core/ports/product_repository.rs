use crate::core::models::{product::{Product, ProductError}, product_category::{ProductCategory, ProductCategoryError}};

pub trait ProductRepository {
    fn create_product(&self, product: Product) -> Result<Product, ProductError>;
    fn find_product_by_id(&self, id: i64) -> Result<Product, ProductError>;
    fn find_all_products(&self) -> Result<Vec<Product>, ProductError>;
    fn update_product(&self, product: Product) -> Result<Product, ProductError>;
    fn delete_product(&self, id: i64) -> Result<(), ProductError>;
    fn find_products_by_name(&self, name: &str) -> Result<Vec<Product>, ProductError>;
    fn update_product_stock(&self, id: i64, new_stock: i32) -> Result<Product, ProductError>;
}

pub trait ProductCategoryRepository {
    fn create_product_category(&self, product_category: ProductCategory) -> Result<ProductCategory, ProductCategoryError>;
    fn find_product_category_by_id(&self, id: i64) -> Result<ProductCategory, ProductCategoryError>;
    fn find_categories_by_product_id(&self, product_id: i64) -> Result<Vec<ProductCategory>, ProductCategoryError>;
    fn find_products_by_category_id(&self, category_id: i64) -> Result<Vec<ProductCategory>, ProductCategoryError>;
    fn delete_product_category(&self, id: i64) -> Result<(), ProductCategoryError>;
    fn delete_product_categories_by_product_id(&self, product_id: i64) -> Result<(), ProductCategoryError>;
    fn delete_product_categories_by_category_id(&self, category_id: i64) -> Result<(), ProductCategoryError>;
}
