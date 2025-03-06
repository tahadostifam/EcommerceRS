use std::{cell::RefCell, ops::DerefMut, rc::Rc};

use crate::{
    adapters::postgres::{
        entities::cart::{CartEntity, CartItemEntity, NewCartEntity, NewCartItemEntity},
        schema::{cart_items, carts},
    },
    core::{
        models::cart::{Cart, CartItem},
        ports::cart_repository::{CartError, CartRepository},
    },
};
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, query_dsl::methods::FilterDsl};

pub struct CartRepositoryImpl {
    conn: Rc<RefCell<PgConnection>>,
}

impl CartRepositoryImpl {
    pub fn new(conn: Rc<RefCell<PgConnection>>) -> Self {
        CartRepositoryImpl { conn }
    }
}

impl CartRepository for CartRepositoryImpl {
    fn create_cart(&mut self, user_id: i64) -> Result<Cart, CartError> {
        let mut conn_borrow = self.conn.borrow_mut();

        let new_cart = NewCartEntity { user_id };

        diesel::insert_into(carts::table)
            .values(&new_cart)
            .get_result::<CartEntity>(conn_borrow.deref_mut())
            .map(|entity| Cart {
                id: entity.id,
                user_id: entity.user_id,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| CartError::DatabaseError)
    }

    fn find_cart_by_id(&mut self, id: i64) -> Result<Cart, CartError> {
        let mut conn_borrow = self.conn.borrow_mut();

        carts::table
            .filter(carts::id.eq(id))
            .first::<CartEntity>(conn_borrow.deref_mut())
            .map(|entity| Cart {
                id: entity.id,
                user_id: entity.user_id,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| CartError::NotFound)
    }

    fn find_carts_by_user_id(&mut self, user_id: i64) -> Result<Cart, CartError> {
        let mut conn_borrow = self.conn.borrow_mut();

        carts::table
            .filter(carts::user_id.eq(user_id))
            .first::<CartEntity>(conn_borrow.deref_mut())
            .map(|entity| Cart {
                id: entity.id,
                user_id: entity.user_id,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| CartError::NotFound)
    }

    fn delete_cart(&mut self, id: i64) -> Result<(), CartError> {
        let mut conn_borrow = self.conn.borrow_mut();

        diesel::delete(carts::table.filter(carts::id.eq(id)))
            .execute(conn_borrow.deref_mut())
            .map(|affected_rows| {
                if affected_rows == 0 {
                    Err(CartError::NotFound)
                } else {
                    Ok(())
                }
            })
            .map_err(|_| CartError::DatabaseError)?
    }

    fn add_cart_item(&mut self, cart_item: CartItem) -> Result<CartItem, CartError> {
        let mut conn_borrow = self.conn.borrow_mut();

        let new_cart_item = NewCartItemEntity {
            cart_id: cart_item.cart_id,
            product_id: cart_item.product_id,
            quantity: cart_item.quantity,
        };

        diesel::insert_into(cart_items::table)
            .values(&new_cart_item)
            .get_result::<CartItemEntity>(conn_borrow.deref_mut())
            .map(|entity| CartItem {
                id: entity.id,
                cart_id: entity.cart_id,
                product_id: entity.product_id,
                quantity: entity.quantity,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| CartError::DatabaseError)
    }

    fn update_cart_item_quantity(
        &mut self,
        cart_item_id: i64,
        new_quantity: i32,
    ) -> Result<CartItem, CartError> {
        let mut conn_borrow = self.conn.borrow_mut();

        diesel::update(cart_items::table.filter(cart_items::id.eq(cart_item_id)))
            .set(cart_items::quantity.eq(new_quantity))
            .get_result::<CartItemEntity>(conn_borrow.deref_mut())
            .map(|entity| CartItem {
                id: entity.id,
                cart_id: entity.cart_id,
                product_id: entity.product_id,
                quantity: entity.quantity,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
            .map_err(|_| CartError::NotFound)
    }

    fn remove_cart_item(&mut self, cart_item_id: i64) -> Result<(), CartError> {
        let mut conn_borrow = self.conn.borrow_mut();

        diesel::delete(cart_items::table.filter(cart_items::id.eq(cart_item_id)))
            .execute(conn_borrow.deref_mut())
            .map(|affected_rows| {
                if affected_rows == 0 {
                    Err(CartError::NotFound)
                } else {
                    Ok(())
                }
            })
            .map_err(|_| CartError::DatabaseError)?
    }
}
