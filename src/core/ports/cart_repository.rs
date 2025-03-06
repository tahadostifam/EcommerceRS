use crate::core::models::cart::{Cart, CartItem};

pub trait CartRepository {
    fn create_cart(&mut self, user_id: i64) -> Result<Cart, CartError>;
    fn find_cart_by_id(&mut self, id: i64) -> Result<Cart, CartError>;
    fn find_carts_by_user_id(&mut self, user_id: i64) -> Result<Cart, CartError>;
    fn delete_cart(&mut self, id: i64) -> Result<(), CartError>;
    fn add_cart_item(&mut self, cart_item: CartItem) -> Result<CartItem, CartError>;
    fn update_cart_item_quantity(&mut self, cart_item_id: i64, new_quantity: i32) -> Result<CartItem, CartError>;
    fn remove_cart_item(&mut self, cart_item_id: i64) -> Result<(), CartError>;
}

#[derive(Debug)]
pub enum CartError {
    NotFound,
    InvalidData,
    DatabaseError,
    Conflict,
    InvalidQuantity,
}