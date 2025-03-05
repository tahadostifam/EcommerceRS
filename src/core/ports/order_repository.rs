use crate::core::models::order::{Order, OrderError, OrderItem, OrderStatus};

pub trait OrderRepository {
    fn create_order(&self, order: Order) -> Result<Order, OrderError>;
    fn find_order_by_id(&self, id: i64) -> Result<Order, OrderError>;
    fn find_orders_by_user_id(&self, user_id: i64) -> Result<Vec<Order>, OrderError>;
    fn update_order_status(&self, id: i64, new_status: OrderStatus) -> Result<Order, OrderError>;
    fn delete_order(&self, id: i64) -> Result<(), OrderError>;
    fn add_order_item(&self, order_item: OrderItem) -> Result<OrderItem, OrderError>;
    fn find_order_items_by_order_id(&self, order_id: i64) -> Result<Vec<OrderItem>, OrderError>;
    fn update_order_item_quantity(&self, order_item_id: i64, new_quantity: i32) -> Result<OrderItem, OrderError>;
    fn remove_order_item(&self, order_item_id: i64) -> Result<(), OrderError>;
}