use diesel::{pg::Pg, prelude::*};
use chrono::NaiveDateTime;
use crate::adapters::postgres::schema::discounts;
use crate::core::models::discount::DiscountType;

#[derive(Debug, Identifiable, Selectable, Queryable, Insertable)]
#[diesel(table_name = discounts)]
#[diesel(check_for_backend(Pg))]
pub struct DiscountEntity {
    pub id: i64,
    pub code: String,
    pub description: String,
    pub discount_type: String, // Stored as string in the database
    pub value: f64,
    pub min_purchase_amount: Option<f64>,
    pub max_discount_amount: Option<f64>,
    pub starts_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = discounts)]
pub struct NewDiscountEntity {
    pub code: String,
    pub description: String,
    pub discount_type: String,
    pub value: f64,
    pub min_purchase_amount: Option<f64>,
    pub max_discount_amount: Option<f64>,
    pub starts_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
}

impl DiscountEntity {
    pub fn to_discount_type(&self) -> DiscountType {
        match self.discount_type.as_str() {
            "Percentage" => DiscountType::Percentage,
            "FixedAmount" => DiscountType::FixedAmount,
            _ => DiscountType::FixedAmount, // Default case
        }
    }
}

impl NewDiscountEntity {
    pub fn from_discount_type(discount_type: &DiscountType) -> String {
        match discount_type {
            DiscountType::Percentage => "Percentage".to_string(),
            DiscountType::FixedAmount => "FixedAmount".to_string(),
        }
    }
}