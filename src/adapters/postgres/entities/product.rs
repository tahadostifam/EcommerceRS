use crate::{adapters::postgres::schema::*, core::models::{category::Category, product::Product}};
use chrono::NaiveDateTime;
use diesel::{pg::Pg, prelude::*};

#[derive(Debug, Clone, Selectable, Queryable, Insertable)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(Pg))]
pub struct ProductEntity {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub product_image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub category_id: Option<i64>,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(Pg))]
pub struct NewProductEntity {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub product_image: Option<String>,
    pub category_id: Option<i64>,
}

impl ProductEntity {
    pub fn to_model(&self, category: Option<Category>) -> Product {
        Product {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            price: self.price,
            stock: self.stock,
            product_image: self.product_image.clone(),
            category,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Identifiable, Selectable, Queryable, Insertable)]
#[diesel(table_name = variations)]
#[diesel(check_for_backend(Pg))]
pub struct Variation {
    pub id: i64,
    pub category_id: i64,
    pub name: String,
}

#[derive(Debug, Identifiable, Selectable, Queryable, Insertable)]
#[diesel(table_name = variation_options)]
#[diesel(check_for_backend(Pg))]
pub struct VariationOption {
    pub id: i64,
    pub variation_id: i64,
    pub value: String,
}
