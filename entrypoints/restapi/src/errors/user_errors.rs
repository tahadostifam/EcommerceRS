use core::fmt;

use actix_web::ResponseError;
use ecommercers::core::models::auth::AuthError;

#[derive(Debug)]
pub struct ActixAuthError(pub AuthError);

impl fmt::Display for ActixAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl ResponseError for ActixAuthError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        todo!();
        // let mut res = actix_web::HttpResponse::new(self.status_code());

        // let mut buf = actix_web::web::BytesMut::new();
        // let _ = std::write!(helpers::MutWriter(&mut buf), "{}", self);

        // let mime = mime::TEXT_PLAIN_UTF_8.try_into_value().unwrap();
        // res.headers_mut().insert(actix_web::http::header::CONTENT_TYPE, mime);

        // res.set_body(actix_web::body::BoxBody::new(buf))
    }
}