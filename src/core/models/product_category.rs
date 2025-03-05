use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductCategory {
    pub id: i64,
    pub product_id: i64,
    pub category_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub enum ProductCategoryError {
    NotFound,
    InvalidData,
    DatabaseError,
    Conflict,
    ProductNotFound,
    CategoryNotFound,
}