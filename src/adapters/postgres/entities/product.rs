use diesel::{pg::Pg, prelude::*};
use chrono::NaiveDateTime;
use crate::adapters::postgres::schema::*;

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