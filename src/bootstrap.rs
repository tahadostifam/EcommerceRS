mod adapters;
mod config;
mod core;

use std::{cell::RefCell, rc::Rc};

use adapters::postgres::{
    adapter::new_postgres_adapter, repos::{cart_repository::CartRepositoryImpl, product_repository::ProductRepositoryImpl},
};

fn main() {
    // Config
    let cfg = config::read();

    // Database Connections
    let pg_conn = Rc::new(RefCell::new(new_postgres_adapter(cfg.database.url)));

    // Repositories
    let mut product_repository = ProductRepositoryImpl::new(Rc::clone(&pg_conn));
    let mut cart_repository = CartRepositoryImpl::new(Rc::clone(&pg_conn));


    println!("EcommerceRS Bootstrap");
}
