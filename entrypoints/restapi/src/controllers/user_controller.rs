use crate::{dto::user_dto::UserRegisterDTO, errors::user_errors::HttpAuthError};
use actix_web::{HttpResponse, Responder, Scope, http::StatusCode, post, web};
use ecommercers::core::services::user_service::UserService;
use std::sync::{Arc, Mutex};

pub fn new_user_controller() -> Scope {
    web::scope("/users").service(register)
}

#[post("/register")]
async fn register(
    user_service_guard: web::Data<Arc<Mutex<UserService>>>,
    data: web::Json<UserRegisterDTO>,
) -> Result<impl Responder, HttpAuthError> {
    let mut user_service = user_service_guard.lock().unwrap();

    match user_service.register(data.0.name, data.0.email, data.0.password) {
        Ok(user) => Ok(HttpResponse::build(StatusCode::OK).json(user)),
        Err(err) => Err(err.into()),
    }
}
