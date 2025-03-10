use std::sync::{Arc, Mutex};

use actix_web::{Responder, Scope, post, web};
use ecommercers::core::services::product_service::ProductService;

pub fn new_product_controller() -> Scope {
    web::scope("/products").service(create_action)
}

#[post("/create")]
async fn create_action(
    user_service_guard: web::Data<Arc<Mutex<ProductService>>>,
) -> impl Responder {
    ""
}
