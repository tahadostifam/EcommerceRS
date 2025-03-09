use std::sync::{Arc, Mutex};

use actix_web::{post, web, Responder, ResponseError, Scope};
use ecommercers::core::{models::user::UserError, services::user_service::UserService};

use crate::dto::user_dto::UserRegisterDTO;

pub fn new_user_controller() -> Scope {
    web::scope("/users")
        .service(register)
}

#[post("/register")]
async fn register(user_service_guard: web::Data<Arc<Mutex<UserService>>>, data: web::Json<UserRegisterDTO>) -> impl Responder {
    let mut user_service = user_service_guard.lock().unwrap(); // FIXME Consider to remote unwrap
    user_service.register(data.0.name, data.0.email, data.0.password).unwrap();

    

    "not implemented yet"
}
