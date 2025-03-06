mod adapters;
mod config;
mod core;

use core::ports::product_repository::ProductRepository;

use adapters::postgres::{
    adapter::new_postgres_adapter, repos::product_repository::ProductRepositoryImpl,
};

fn main() {
    let cfg = config::read();

    let pg_conn = new_postgres_adapter(cfg.database.url);
    let mut product_repository = ProductRepositoryImpl::new(pg_conn);
    let _ = product_repository
        .create_product(String::from("product 1"), String::from("desc"), 10.5, 3)
        .unwrap();

    println!("EcommerceRS Bootstrap");
}
