mod adapters;
mod config;
mod core;

use core::ports::product_repository::ProductRepository;

use adapters::postgres::{
    adapter::new_postgres_adapter, repos::product_repository::ProductRepositoryImpl,
};

fn main() {
    // Config
    let cfg = config::read();

    // Database Connections
    let pg_conn = new_postgres_adapter(cfg.database.url);

    // Repositories
    let mut product_repository = ProductRepositoryImpl::new(pg_conn);


    println!("EcommerceRS Bootstrap");
}
