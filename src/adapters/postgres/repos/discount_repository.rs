use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use chrono::{NaiveDateTime, Utc};
use diesel::{
    ExpressionMethods, PgConnection, RunQueryDsl,
    query_dsl::methods::FilterDsl,
};

use crate::{
    adapters::postgres::{
        entities::discount::{DiscountEntity, NewDiscountEntity},
        schema::discounts,
    },
    core::{
        models::discount::{Discount, DiscountError, DiscountType},
        ports::discount_repository::DiscountRepository,
    },
};

pub struct DiscountRepositoryImpl {
    conn: Arc<Mutex<PgConnection>>,
}

impl DiscountRepositoryImpl {
    pub fn new(conn: Arc<Mutex<PgConnection>>) -> Self {
        DiscountRepositoryImpl { conn }
    }

    fn map_entity_to_model(&self, entity: DiscountEntity) -> Discount {
        Discount {
            id: entity.id,
            code: entity.code,
            description: entity.description,
            discount_type: entity.to_discount_type(),
            value: entity.value,
            min_purchase_amount: entity.min_purchase_amount,
            max_discount_amount: entity.max_discount_amount,
            starts_at: entity.starts_at,
            expires_at: entity.expires_at,
            is_active: entity.is_active,
            usage_limit: entity.usage_limit,
            usage_count: entity.usage_count,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

impl DiscountRepository for DiscountRepositoryImpl {
    fn create_discount(
        &mut self,
        code: String,
        description: String,
        discount_type: DiscountType,
        value: f64,
        min_purchase_amount: Option<f64>,
        max_discount_amount: Option<f64>,
        starts_at: NaiveDateTime,
        expires_at: Option<NaiveDateTime>,
        is_active: bool,
        usage_limit: Option<i32>,
    ) -> Result<Discount, DiscountError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        let entity = NewDiscountEntity {
            code,
            description,
            discount_type: NewDiscountEntity::from_discount_type(&discount_type),
            value,
            min_purchase_amount,
            max_discount_amount,
            starts_at,
            expires_at,
            is_active,
            usage_limit,
            usage_count: 0,
        };

        diesel::insert_into(discounts::table)
            .values(&entity)
            .get_result::<DiscountEntity>(conn_borrow.deref_mut())
            .map(|entity| self.map_entity_to_model(entity))
            .map_err(|_| DiscountError::DatabaseError)
    }

    fn find_discount_by_id(&mut self, id: i64) -> Result<Discount, DiscountError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        discounts::table
            .filter(discounts::id.eq(id))
            .first::<DiscountEntity>(conn_borrow.deref_mut())
            .map(|entity| self.map_entity_to_model(entity))
            .map_err(|_| DiscountError::NotFound)
    }

    fn find_discount_by_code(&mut self, code: String) -> Result<Discount, DiscountError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        discounts::table
            .filter(discounts::code.eq(code))
            .first::<DiscountEntity>(conn_borrow.deref_mut())
            .map(|entity| self.map_entity_to_model(entity))
            .map_err(|_| DiscountError::NotFound)
    }

    fn find_all_discounts(&mut self) -> Result<Vec<Discount>, DiscountError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        discounts::table
            .load::<DiscountEntity>(conn_borrow.deref_mut())
            .map(|entities| {
                entities
                    .into_iter()
                    .map(|entity| self.map_entity_to_model(entity))
                    .collect()
            })
            .map_err(|_| DiscountError::DatabaseError)
    }

    fn find_active_discounts(&mut self) -> Result<Vec<Discount>, DiscountError> {
        let mut conn_borrow = self.conn.lock().unwrap();
        let now = Utc::now().naive_utc();

        discounts::table
            .filter(discounts::is_active.eq(true))
            .filter(discounts::starts_at.le(now))
            .filter(
                discounts::expires_at
                    .is_null()
                    .or(discounts::expires_at.gt(now)),
            )
            .load::<DiscountEntity>(conn_borrow.deref_mut())
            .map(|entities| {
                entities
                    .into_iter()
                    .map(|entity| self.map_entity_to_model(entity))
                    .collect()
            })
            .map_err(|_| DiscountError::DatabaseError)
    }

    fn update_discount(
        &mut self,
        id: i64,
        code: String,
        description: String,
        discount_type: DiscountType,
        value: f64,
        min_purchase_amount: Option<f64>,
        max_discount_amount: Option<f64>,
        starts_at: NaiveDateTime,
        expires_at: Option<NaiveDateTime>,
        is_active: bool,
        usage_limit: Option<i32>,
    ) -> Result<Discount, DiscountError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        // First, check if the discount exists
        let existing_discount = discounts::table
            .filter(discounts::id.eq(id))
            .first::<DiscountEntity>(conn_borrow.deref_mut())
            .map_err(|_| DiscountError::NotFound)?;

        let entity = NewDiscountEntity {
            code,
            description,
            discount_type: NewDiscountEntity::from_discount_type(&discount_type),
            value,
            min_purchase_amount,
            max_discount_amount,
            starts_at,
            expires_at,
            is_active,
            usage_limit,
            usage_count: existing_discount.usage_count, // Preserve the usage count
        };

        diesel::update(discounts::table.filter(discounts::id.eq(id)))
            .set(&entity)
            .get_result::<DiscountEntity>(conn_borrow.deref_mut())
            .map(|entity| self.map_entity_to_model(entity))
            .map_err(|_| DiscountError::DatabaseError)
    }

    fn update_discount_usage_count(&mut self, id: i64, new_count: i32) -> Result<Discount, DiscountError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        diesel::update(discounts::table.filter(discounts::id.eq(id)))
            .set(discounts::usage_count.eq(new_count))
            .get_result::<DiscountEntity>(conn_borrow.deref_mut())
            .map(|entity| self.map_entity_to_model(entity))
            .map_err(|_| DiscountError::DatabaseError)
    }

    fn delete_discount(&mut self, id: i64) -> Result<(), DiscountError> {
        let mut conn_borrow = self.conn.lock().unwrap();

        diesel::delete(discounts::table.filter(discounts::id.eq(id)))
            .execute(conn_borrow.deref_mut())
            .map(|affected_rows| {
                if affected_rows == 0 {
                    Err(DiscountError::NotFound)
                } else {
                    Ok(())
                }
            })
            .map_err(|_| DiscountError::DatabaseError)?
    }
}