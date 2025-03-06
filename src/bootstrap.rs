mod adapters;
mod config;
mod core;

use std::{cell::RefCell, rc::Rc};

fn main() {
    // Config
    let cfg = config::read();

    // Database Connections
    let pg_conn = Rc::new(RefCell::new(
        adapters::postgres::adapter::new_postgres_adapter(cfg.database.url),
    ));

    // Repositories
    let mut product_repository =
        adapters::postgres::repos::product_repository::ProductRepositoryImpl::new(Rc::clone(
            &pg_conn,
        ));

    let mut cart_repository =
        adapters::postgres::repos::cart_repository::CartRepositoryImpl::new(Rc::clone(&pg_conn));

    let mut order_repository =
        adapters::postgres::repos::order_repository::OrderRepositoryImpl::new(Rc::clone(&pg_conn));

    println!("EcommerceRS Bootstrap");
}
