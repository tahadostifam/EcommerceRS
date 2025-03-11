use ecommercers::core::models::product::Product;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductCreateDTO {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub product_image: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductCreatedDTO {
    pub message: String,
    pub product: Product,
}
