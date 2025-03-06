use std::{fmt, str::FromStr};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Error,
}

impl FromStr for OrderStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(OrderStatus::Pending),
            "Processing" => Ok(OrderStatus::Processing),
            "Shipped" => Ok(OrderStatus::Shipped),
            "Delivered" => Ok(OrderStatus::Delivered),
            "Cancelled" => Ok(OrderStatus::Cancelled),
            "Error" => Ok(OrderStatus::Error),
            _ => Err(format!("'{}' is not a valid OrderStatus", s)),
        }
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "Pending"),
            OrderStatus::Processing => write!(f, "Processing"),
            OrderStatus::Shipped => write!(f, "Shipped"),
            OrderStatus::Delivered => write!(f, "Delivered"),
            OrderStatus::Cancelled => write!(f, "Cancelled"),
            OrderStatus::Error => write!(f, "Error"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: i64,
    pub user_id: i64,
    pub total_amount: f64,
    pub status: OrderStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderItem {
    pub id: i64,
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub price_at_time_of_order: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub enum OrderError {
    NotFound,
    InvalidData,
    DatabaseError,
    Conflict,
    InvalidStatusTransition,
}
