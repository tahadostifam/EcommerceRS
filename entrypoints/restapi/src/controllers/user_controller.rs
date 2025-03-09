use actix_web::{web, Scope};

pub fn new_user_controller() -> Scope {
    web::scope("/users")
}
