use diesel::{pg::Pg, prelude::*};
use chrono::NaiveDateTime;
use crate::adapters::postgres::schema::*;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = order_items)]
#[diesel(check_for_backend(Pg))]
pub struct OrderItemEntity {
    pub id: i64,
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub price_at_time_of_order: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = orders)]
#[diesel(check_for_backend(Pg))]
pub struct OrderEntity {
    pub id: i64,
    pub user_id: i64,
    pub total_amount: f64,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}