use crate::adapters;
use crate::config::{self, Config};
use crate::core::services::email_service::new_email_service_devel;
use crate::core::services::user_service::{UserService, new_user_service};
use std::sync::{Arc, Mutex};

pub struct Services {
    pub cfg: Arc<Mutex<Config>>,
    pub user_service: Arc<Mutex<UserService>>,
}

pub fn bootstrap_services() -> Services {
    // Config
    let cfg = config::read();

    // Database Connections
    let pg_conn = Arc::new(Mutex::new(
        adapters::postgres::adapter::new_postgres_adapter(cfg.postgres.url.clone()),
    ));

    let redis_conn = Arc::new(Mutex::new(adapters::redis::adapter::new_redis_adapter(
        cfg.redis.url.clone(),
    )));

    // Repositories
    let mut auth_repository = Arc::new(Mutex::new(
        adapters::redis::repos::auth_repository::AuthRepositoryImpl::new(redis_conn),
    ));

    let mut user_repository = Arc::new(Mutex::new(
        adapters::postgres::repos::user_repository::UserRepositoryImpl::new(pg_conn.clone()),
    ));

    let mut product_repository = Arc::new(Mutex::new(
        adapters::postgres::repos::product_repository::ProductRepositoryImpl::new(pg_conn.clone()),
    ));

    let mut cart_repository = Arc::new(Mutex::new(
        adapters::postgres::repos::cart_repository::CartRepositoryImpl::new(pg_conn.clone()),
    ));

    let mut order_repository = Arc::new(Mutex::new(
        adapters::postgres::repos::order_repository::OrderRepositoryImpl::new(pg_conn.clone()),
    ));

    // Services
    let email_service = new_email_service_devel();
    let user_service = new_user_service(
        cfg.jwt.secret.clone(),
        auth_repository,
        user_repository,
        Arc::new(Mutex::new(email_service)),
    );

    println!("# EcommerceRS");

    Services {
        cfg: Arc::new(Mutex::new(cfg)),
        user_service: Arc::new(Mutex::new(user_service)),
        discount_service: Arc::new(Mutex::new(discount_service)),
    }
}