use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub enum ProductError {
    NotFound,
    InvalidData,
    DatabaseError,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Variation {
    pub id: i64,
    pub category_id: i64,
    pub name: String, 
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VariationOption {
    pub id: i64,
    pub variation_id: i64,
    pub value: String, 
}