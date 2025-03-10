use std::sync::{Arc, Mutex};
use chrono::{NaiveDateTime, Utc};

use crate::core::models::discount::{Discount, DiscountError, DiscountType};
use crate::core::ports::discount_repository::DiscountRepository;

#[derive(Clone)]
pub struct DiscountService {
    pub(crate) discount_repo: Arc<Mutex<dyn DiscountRepository>>,
}

pub fn new_discount_service(
    discount_repo: Arc<Mutex<dyn DiscountRepository>>,
) -> DiscountService {
    DiscountService {
        discount_repo
    }
}

impl DiscountService {
    pub fn create_discount(
        &self,
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
        // Validate discount data
        if code.is_empty() {
            return Err(DiscountError::InvalidData);
        }

        if value <= 0.0 {
            return Err(DiscountError::InvalidData);
        }

        if discount_type == DiscountType::Percentage && value > 100.0 {
            return Err(DiscountError::InvalidData);
        }

        let mut repo = self.discount_repo.lock().unwrap();
        repo.create_discount(
            code,
            description,
            discount_type,
            value,
            min_purchase_amount,
            max_discount_amount,
            starts_at,
            expires_at,
            is_active,
            usage_limit,
        )
    }

    pub fn get_discount_by_id(&self, id: i64) -> Result<Discount, DiscountError> {
        let mut repo = self.discount_repo.lock().unwrap();
        repo.find_discount_by_id(id)
    }

    pub fn get_discount_by_code(&self, code: String) -> Result<Discount, DiscountError> {
        let mut repo = self.discount_repo.lock().unwrap();
        repo.find_discount_by_code(code)
    }

    pub fn get_all_discounts(&self) -> Result<Vec<Discount>, DiscountError> {
        let mut repo = self.discount_repo.lock().unwrap();
        repo.find_all_discounts()
    }

    pub fn get_active_discounts(&self) -> Result<Vec<Discount>, DiscountError> {
        let mut repo = self.discount_repo.lock().unwrap();
        repo.find_active_discounts()
    }

    pub fn update_discount(
        &self,
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
        // Validate discount data
        if code.is_empty() {
            return Err(DiscountError::InvalidData);
        }

        if value <= 0.0 {
            return Err(DiscountError::InvalidData);
        }

        if discount_type == DiscountType::Percentage && value > 100.0 {
            return Err(DiscountError::InvalidData);
        }

        let mut repo = self.discount_repo.lock().unwrap();
        repo.update_discount(
            id,
            code,
            description,
            discount_type,
            value,
            min_purchase_amount,
            max_discount_amount,
            starts_at,
            expires_at,
            is_active,
            usage_limit,
        )
    }

    pub fn delete_discount(&self, id: i64) -> Result<(), DiscountError> {
        let mut repo = self.discount_repo.lock().unwrap();
        repo.delete_discount(id)
    }

    pub fn apply_discount(&self, code: String, purchase_amount: f64) -> Result<f64, DiscountError> {
        let mut repo = self.discount_repo.lock().unwrap();
        let discount = repo.find_discount_by_code(code)?;

        // Check if discount is active
        if !discount.is_active {
            return Err(DiscountError::InactiveDiscount);
        }

        // Check if discount has expired
        let now = Utc::now().naive_utc();
        if discount.starts_at > now {
            return Err(DiscountError::Expired);
        }

        if let Some(expires_at) = discount.expires_at {
            if expires_at < now {
                return Err(DiscountError::Expired);
            }
        }

        // Check if usage limit has been reached
        if let Some(usage_limit) = discount.usage_limit {
            if discount.usage_count >= usage_limit {
                return Err(DiscountError::UsageLimitReached);
            }
        }

        // Check if minimum purchase amount is met
        if let Some(min_purchase) = discount.min_purchase_amount {
            if purchase_amount < min_purchase {
                return Err(DiscountError::MinPurchaseNotMet);
            }
        }

        // Calculate discount amount
        let discount_amount = match discount.discount_type {
            DiscountType::Percentage => purchase_amount * (discount.value / 100.0),
            DiscountType::FixedAmount => discount.value,
        };

        // Apply maximum discount amount if specified
        let final_discount_amount = if let Some(max_discount) = discount.max_discount_amount {
            discount_amount.min(max_discount)
        } else {
            discount_amount
        };

        // Increment usage count
        let new_count = discount.usage_count + 1;
        repo.update_discount_usage_count(discount.id, new_count)?;

        Ok(final_discount_amount)
    }
}