use crate::adapters::postgres::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = cart_items)]
#[diesel(check_for_backend(Pg))]
pub struct CartItemEntity {
    pub id: i64,
    pub cart_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = cart_items)]
pub struct NewCartItemEntity {
    pub cart_id: i64,
    pub product_id: i64,
    pub quantity: i32,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = carts)]
#[diesel(check_for_backend(Pg))]
pub struct CartEntity {
    pub id: i64,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = carts)]
pub struct NewCartEntity {
    pub user_id: i64,
}