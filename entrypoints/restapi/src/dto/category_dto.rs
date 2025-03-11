use ecommercers::core::models::category::Category;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryCreateDTO {
    pub name: String,
    pub description: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryCreatedDTO {
    pub category: Category,
    pub message: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryDeleteDTO {
    pub category_id: i64
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryGetDTO {
    pub category_id: i64
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryUpdateDTO {
    pub category_id: i64,
    pub name: String,
    pub description: String,
    pub parent_id: Option<i64>,
}
