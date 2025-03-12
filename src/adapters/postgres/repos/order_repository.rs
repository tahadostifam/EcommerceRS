use crate::adapters::postgres::entities::order::{OrderEntity, OrderItemEntity};
use crate::core::ports::order_repository::OrderRepository;
use crate::{
    adapters::postgres::schema::{order_items, orders},
    core::models::order::{Order, OrderError, OrderItem, OrderStatus},
};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use std::ops::DerefMut;
use std::str::FromStr;

pub struct OrderRepositoryImpl {
    conn: Pool<ConnectionManager<PgConnection>>,
}

impl OrderRepositoryImpl {
    pub fn new(conn: Pool<ConnectionManager<PgConnection>>) -> Self {
        OrderRepositoryImpl { conn }
    }
}

impl OrderRepository for OrderRepositoryImpl {
    fn create_order(&mut self, order: Order) -> Result<Order, OrderError> {
        let mut conn = self.conn.get().unwrap();

        diesel::insert_into(orders::table)
            .values(OrderEntity {
                id: order.id,
                user_id: order.user_id,
                total_amount: order.total_amount,
                status: order.status.to_string(),
                created_at: order.created_at,
                updated_at: order.updated_at,
            })
            .get_result::<OrderEntity>(conn.deref_mut())
            .map(|entity| Order {
                id: entity.id,
                user_id: entity.user_id,
                total_amount: entity.total_amount,
                status: order.status,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| OrderError::DatabaseError)
    }

    fn find_order_by_id(&mut self, id: i64) -> Result<Order, OrderError> {
        let mut conn = self.conn.get().unwrap();
        

        orders::table
            .filter(orders::id.eq(id))
            .first::<OrderEntity>(conn.deref_mut())
            .map(|entity| Order {
                id: entity.id,
                user_id: entity.user_id,
                total_amount: entity.total_amount,
                status: OrderStatus::from_str(&entity.status).unwrap_or(OrderStatus::Error),
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| OrderError::NotFound)
    }

    fn find_orders_by_user_id(&mut self, user_id: i64) -> Result<Vec<Order>, OrderError> {
        let mut conn = self.conn.get().unwrap();
        

        orders::table
            .filter(orders::user_id.eq(user_id))
            .load::<OrderEntity>(conn.deref_mut())
            .map(|entities| {
                entities
                    .into_iter()
                    .map(|entity| Order {
                        id: entity.id,
                        user_id: entity.user_id,
                        total_amount: entity.total_amount,
                        status: OrderStatus::from_str(&entity.status).unwrap_or(OrderStatus::Error),
                        created_at: entity.created_at,
                        updated_at: entity.updated_at,
                    })
                    .collect()
            })
            .map_err(|_| OrderError::DatabaseError)
    }

    fn update_order_status(
        &mut self,
        id: i64,
        new_status: OrderStatus,
    ) -> Result<Order, OrderError> {
        let mut conn = self.conn.get().unwrap();
        

        diesel::update(orders::table.filter(orders::id.eq(id)))
            .set(orders::status.eq(new_status.to_string()))
            .get_result::<OrderEntity>(conn.deref_mut())
            .map(|entity| Order {
                id: entity.id,
                user_id: entity.user_id,
                total_amount: entity.total_amount,
                status: OrderStatus::from_str(&entity.status).unwrap_or(OrderStatus::Error),
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| OrderError::NotFound)
    }

    fn delete_order(&mut self, id: i64) -> Result<(), OrderError> {
        let mut conn = self.conn.get().unwrap();
        
        diesel::delete(orders::table.filter(orders::id.eq(id)))
            .execute(conn.deref_mut())
            .map(|affected_rows| {
                if affected_rows == 0 {
                    Err(OrderError::NotFound)
                } else {
                    Ok(())
                }
            })
            .map_err(|_| OrderError::DatabaseError)?
    }

    fn add_order_item(&mut self, order_item: OrderItem) -> Result<OrderItem, OrderError> {
        let mut conn = self.conn.get().unwrap();
        
        diesel::insert_into(order_items::table)
            .values(&OrderItemEntity {
                id: order_item.id,
                order_id: order_item.order_id,
                product_id: order_item.product_id,
                quantity: order_item.quantity,
                price_at_time_of_order: order_item.price_at_time_of_order,
                created_at: order_item.created_at,
                updated_at: order_item.updated_at,
            })
            .get_result::<OrderItemEntity>(conn.deref_mut())
            .map(|entity| OrderItem {
                id: entity.id,
                order_id: entity.order_id,
                product_id: entity.product_id,
                quantity: entity.quantity,
                price_at_time_of_order: entity.price_at_time_of_order,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| OrderError::DatabaseError)
    }

    fn find_order_items_by_order_id(
        &mut self,
        order_id: i64,
    ) -> Result<Vec<OrderItem>, OrderError> {
        let mut conn = self.conn.get().unwrap();
        
        order_items::table
            .filter(order_items::order_id.eq(order_id))
            .load::<OrderItemEntity>(conn.deref_mut())
            .map(|entities| {
                entities
                    .into_iter()
                    .map(|entity| OrderItem {
                        id: entity.id,
                        order_id: entity.order_id,
                        product_id: entity.product_id,
                        quantity: entity.quantity,
                        price_at_time_of_order: entity.price_at_time_of_order,
                        created_at: entity.created_at,
                        updated_at: entity.updated_at,
                    })
                    .collect()
            })
            .map_err(|_| OrderError::DatabaseError)
    }

    fn update_order_item_quantity(
        &mut self,
        order_item_id: i64,
        new_quantity: i32,
    ) -> Result<OrderItem, OrderError> {
        let mut conn = self.conn.get().unwrap();
        
        diesel::update(order_items::table.filter(order_items::id.eq(order_item_id)))
            .set(order_items::quantity.eq(new_quantity))
            .get_result::<OrderItemEntity>(conn.deref_mut())
            .map(|entity| OrderItem {
                id: entity.id,
                order_id: entity.order_id,
                product_id: entity.product_id,
                quantity: entity.quantity,
                price_at_time_of_order: entity.price_at_time_of_order,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| OrderError::NotFound)
    }

    fn remove_order_item(&mut self, order_item_id: i64) -> Result<(), OrderError> {
        let mut conn = self.conn.get().unwrap();
        
        diesel::delete(order_items::table.filter(order_items::id.eq(order_item_id)))
            .execute(conn.deref_mut())
            .map(|affected_rows| {
                if affected_rows == 0 {
                    Err(OrderError::NotFound)
                } else {
                    Ok(())
                }
            })
            .map_err(|_| OrderError::DatabaseError)?
    }
}
