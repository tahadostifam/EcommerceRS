use crate::{
    dto::user_dto::{UserLoggedInDTO, UserLoginDTO, UserRegisterDTO},
    errors::{SimpleMessage, user_errors::HttpAuthError},
};
use actix_web::{
    HttpResponse, Responder, Scope,
    http::{StatusCode, header::ContentType},
    post, web,
};
use ecommercers::core::services::user_service::UserService;
use std::sync::{Arc, Mutex};

pub fn new_user_controller() -> Scope {
    web::scope("/users").service(register).service(login)
}

#[post("/register")]
async fn register(
    user_service_guard: web::Data<Arc<Mutex<UserService>>>,
    data: web::Json<UserRegisterDTO>,
) -> Result<impl Responder, HttpAuthError> {
    let mut user_service = user_service_guard.lock().unwrap();

    match user_service.register(
        data.0.first_name,
        data.0.last_name,
        data.0.email,
        data.0.password,
    ) {
        Ok(_) => Ok(HttpResponse::build(StatusCode::OK)
            .insert_header(ContentType::json())
            .body(
                serde_json::to_string(&SimpleMessage {
                    message: "verification email sent successfully".to_string(),
                })
                .unwrap(),
            )),
        Err(err) => Err(err.into()),
    }
}

#[post("/login")]
async fn login(
    user_service_guard: web::Data<Arc<Mutex<UserService>>>,
    data: web::Json<UserLoginDTO>,
) -> Result<impl Responder, HttpAuthError> {
    let mut user_service = user_service_guard.lock().unwrap();

    match user_service.login(data.0.email, data.0.password) {
        Ok((user, refresh_token, access_token)) => Ok(HttpResponse::build(StatusCode::OK)
            .insert_header(ContentType::json())
            .body(
                serde_json::to_string(&UserLoggedInDTO {
                    user,
                    refresh_token,
                    access_token,
                })
                .unwrap(),
            )),
        Err(err) => Err(err.into()),
    }
}
