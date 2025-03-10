use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Discount {
    pub id: i64,
    pub code: String,
    pub description: String,
    pub discount_type: DiscountType,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DiscountType {
    Percentage,
    FixedAmount,
}

#[derive(Debug)]
pub enum DiscountError {
    NotFound,
    InvalidData,
    DatabaseError,
    Expired,
    InactiveDiscount,
    UsageLimitReached,
    MinPurchaseNotMet,
}