use crate::adapters;
use crate::config::{self, Config};
use crate::core::services::user_service::{UserService, new_user_service};
use std::{cell::RefCell, rc::Rc};

pub struct Services {
    pub cfg: Rc<RefCell<Config>>,
    pub user_service: Rc<RefCell<UserService>>,
}

pub fn bootstrap_services() -> Services {
    // Config
    let cfg = config::read();

    // Database Connections
    let pg_conn = Rc::new(RefCell::new(
        adapters::postgres::adapter::new_postgres_adapter(cfg.postgres.url.clone()),
    ));

    let redis_conn = Rc::new(RefCell::new(adapters::redis::adapter::new_redis_adapter(
        cfg.redis.url.clone(),
    )));

    // Repositories
    let mut auth_repository = Rc::new(RefCell::new(
        adapters::redis::repos::auth_repository::AuthRepositoryImpl::new(redis_conn),
    ));

    let mut user_repository = Rc::new(RefCell::new(
        adapters::postgres::repos::user_repository::UserRepositoryImpl::new(Rc::clone(&pg_conn)),
    ));

    let mut product_repository = Rc::new(RefCell::new(
        adapters::postgres::repos::product_repository::ProductRepositoryImpl::new(Rc::clone(
            &pg_conn,
        )),
    ));

    let mut cart_repository = Rc::new(RefCell::new(
        adapters::postgres::repos::cart_repository::CartRepositoryImpl::new(Rc::clone(&pg_conn)),
    ));

    let mut order_repository = Rc::new(RefCell::new(
        adapters::postgres::repos::order_repository::OrderRepositoryImpl::new(Rc::clone(&pg_conn)),
    ));

    // Services
    let user_service = new_user_service(auth_repository, user_repository);

    Services {
        cfg: Rc::new(RefCell::new(cfg)),
        user_service: Rc::new(RefCell::new(user_service)),
    }
}

fn main() {
    unimplemented!();
}
