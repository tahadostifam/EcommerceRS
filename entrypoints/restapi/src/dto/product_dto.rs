use ecommercers::core::models::product::Product;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductCreateDTO {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub product_image: Option<String>,
    pub category_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductUpdateDTO {
    pub product_id: i64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub product_image: Option<String>,
    pub category_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductUpdatedDTO {
    pub message: String,
    pub product: Product,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductCreatedDTO {
    pub message: String,
    pub product: Product,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductGetDTO {
    pub product_id: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductSearchDTO {
    pub search: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductDeleteDTO {
    pub product_id: i64,
}