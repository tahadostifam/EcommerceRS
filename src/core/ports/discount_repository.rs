use crate::core::models::discount::{Discount, DiscountError, DiscountType};
use chrono::NaiveDateTime;

pub trait DiscountRepository: Send + Sync {
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
    ) -> Result<Discount, DiscountError>;

    fn find_discount_by_id(&mut self, id: i64) -> Result<Discount, DiscountError>;

    fn find_discount_by_code(&mut self, code: String) -> Result<Discount, DiscountError>;

    fn find_all_discounts(&mut self) -> Result<Vec<Discount>, DiscountError>;

    fn find_active_discounts(&mut self) -> Result<Vec<Discount>, DiscountError>;

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
    ) -> Result<Discount, DiscountError>;

    fn update_discount_usage_count(&mut self, id: i64, new_count: i32) -> Result<Discount, DiscountError>;

    fn delete_discount(&mut self, id: i64) -> Result<(), DiscountError>;
}