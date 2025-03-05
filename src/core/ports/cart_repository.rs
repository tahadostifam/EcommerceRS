use crate::core::models::cart::{Cart, CartItem};

#[derive(Debug)]
pub enum CartError {
    NotFound,
    InvalidData,
    DatabaseError,
    Conflict,
    InvalidQuantity,
}

pub trait CartRepository {
    fn create_cart(&self, user_id: i64) -> Result<Cart, CartError>;
    fn find_cart_by_id(&self, id: i64) -> Result<Cart, CartError>;
    fn find_cart_by_user_id(&self, user_id: i64) -> Result<Cart, CartError>;
    fn delete_cart(&self, id: i64) -> Result<(), CartError>;
    fn add_cart_item(&self, cart_item: CartItem) -> Result<CartItem, CartError>;
    fn find_cart_items_by_cart_id(&self, cart_id: i64) -> Result<Vec<CartItem>, CartError>;
    fn update_cart_item_quantity(&self, cart_item_id: i64, new_quantity: i32) -> Result<CartItem, CartError>;
    fn remove_cart_item(&self, cart_item_id: i64) -> Result<(), CartError>;
}