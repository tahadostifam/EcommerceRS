use diesel::{pg::Pg, prelude::*};
use chrono::NaiveDateTime;
use crate::adapters::postgres::schema::*;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = cart_items)]
#[diesel(check_for_backend(Pg))]
pub struct CartItemEntity {
    pub id: Option<i64>,
    pub cart_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = carts)]
#[diesel(check_for_backend(Pg))]
pub struct CartEntity {
    pub id: Option<i64>,
    pub user_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = categories)]
#[diesel(check_for_backend(Pg))]
pub struct CategoryEntity {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = order_items)]
#[diesel(check_for_backend(Pg))]
pub struct OrderItemEntity {
    pub id: Option<i64>,
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
    pub id: Option<i64>,
    pub user_id: i64,
    pub total_amount: f64,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Identifiable, Selectable, Queryable, Insertable)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(Pg))]
pub struct ProductEntity {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = products)]
pub struct NewProductEntity {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct UserEntity {
    pub id: Option<i64>,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}