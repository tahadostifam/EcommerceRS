use crate::{
    dto::user_dto::{
        UserLoggedInDTO, UserLoginDTO, UserLogoutDTO, UserNewAccessTokenDTO, UserRefreshTokenDTO, UserRegisterDTO
    },
    errors::{user_errors::HttpAuthError, ErrorMessage, SimpleMessage},
};
use actix_web::{
    HttpRequest, HttpResponse, Responder, Scope,
    http::{StatusCode, header::ContentType},
    post, web,
};
use ecommercers::core::services::user_service::UserService;
use std::sync::{Arc, Mutex};

pub fn new_user_controller() -> Scope {
    web::scope("/users")
        .service(register_action)
        .service(login_action)
        .service(refresh_token_action)
        .service(authorization_action)
        .service(logout_action)
}

#[post("/register")]
async fn register_action(
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
async fn login_action(
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

#[post("/refresh_token")]
async fn refresh_token_action(
    user_service_guard: web::Data<Arc<Mutex<UserService>>>,
    data: web::Json<UserRefreshTokenDTO>,
) -> Result<impl Responder, HttpAuthError> {
    let mut user_service = user_service_guard.lock().unwrap();

    match user_service.refresh_token(data.0.refresh_token) {
        Ok(access_token) => Ok(HttpResponse::build(StatusCode::OK)
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&UserNewAccessTokenDTO { access_token }).unwrap())),
        Err(err) => Err(err.into()),
    }
}

#[post("/authorization")]
async fn authorization_action(
    user_service_guard: web::Data<Arc<Mutex<UserService>>>,
    req: HttpRequest,
) -> Result<impl Responder, HttpAuthError> {
    let mut user_service = user_service_guard.lock().unwrap();

    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let access_token = auth_str[7..].to_string(); // Extract token after "Bearer "

                let user = user_service.authorization(access_token)?;

                return Ok(HttpResponse::Ok()
                    .insert_header(ContentType::json())
                    .body(serde_json::to_string(&user).unwrap()));
            } else {
                return Ok(HttpResponse::BadRequest().json(ErrorMessage {
                    error: "invalid authorization header format".to_string(),
                }));
            }
        } else {
            return Ok(HttpResponse::BadRequest().json(ErrorMessage {
                error: "invalid authorization header value".to_string(),
            }));
        }
    }

    Ok(HttpResponse::BadRequest().json(ErrorMessage {
        error: "authorization header required".to_string(),
    }))
}

#[post("/logout")]
async fn logout_action(
    user_service_guard: web::Data<Arc<Mutex<UserService>>>,
    data: web::Json<UserLogoutDTO>,
) -> Result<impl Responder, HttpAuthError> {
    let mut user_service = user_service_guard.lock().unwrap();

    match user_service.logout(data.0.refresh_token) {
        Ok(_) => Ok(HttpResponse::build(StatusCode::OK)
            .insert_header(ContentType::json())
            .body(
                serde_json::to_string(&SimpleMessage {
                    message: "logged out".to_string()
                })
                .unwrap(),
            )),
        Err(err) => Err(err.into()),
    }
}