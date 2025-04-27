use std::sync::Arc;
use actix_web::web;
use actix_web::Error;
use actix_web::body::MessageBody;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::Next;
use ecommercers::core::services::user_service::UserService;
use futures_util::lock::Mutex;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let user_service_guard = req.app_data::<Arc<Mutex<UserService>>>();
    dbg!(user_service_guard);

    next.call(req).await
}

// use crate::errors::user_errors::HttpAuthError;
// use actix_web::{
//     Error, ResponseError,
//     body::{EitherBody, MessageBody},
//     dev::{Service, ServiceRequest, ServiceResponse, Transform},
//     http::header::AUTHORIZATION,
//     web,
// };
// use ecommercers::{config::Config, core::{models::user, services::user_service::UserService}};
// use futures_util::{
//     future::{Ready, ok},
//     lock::Mutex,
// };
// use std::{cell::RefCell, pin::Pin, rc::Rc, sync::Arc};

// pub struct Authentication;

// impl<S, B> Transform<S, ServiceRequest> for Authentication
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
//     B: MessageBody,
// {
//     type Response = ServiceResponse<EitherBody<B>>;
//     type Error = Error;
//     type InitError = ();
//     type Transform = AuthenticationMiddleware<S>;
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ok(AuthenticationMiddleware {
//             service: Rc::new(RefCell::new(service)),
//         })
//     }
// }

// pub struct AuthenticationMiddleware<S> {
//     service: Rc<RefCell<S>>,
// }

// impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
//     B: MessageBody,
// {
//     type Response = ServiceResponse<EitherBody<B>>;
//     type Error = S::Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

//     fn poll_ready(
//         &self,
//         ctx: &mut core::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(ctx)
//     }

//     fn call(&self, req: ServiceRequest) -> Self::Future {
//         if let Some(auth_header) = req.headers().get(AUTHORIZATION).cloned() {
//             let service = self.service.clone();
//             let access_token = auth_header.to_str().unwrap()[7..].to_string();
//             let user_service_app_data = req
//                 .app_data::<web::Data<Arc<Mutex<Config>>>>()
//                 .cloned();

//             dbg!(user_service_app_data);
//             // let fut = async move {
//             //     let mut user_service = user_service_app_data.lock().await;
//             //     if let Ok(user) = user_service.authorization(access_token) {
//             //         dbg!(user.clone());

//             //         service.call(req).await.map(|res| res.map_into_left_body())
//             //     } else {
//             //         Ok(req.into_response(
//             //             HttpAuthError::InvalidCredentials
//             //                 .error_response()
//             //                 .map_into_right_body(),
//             //         ))
//             //     }
//             // };

//             // Box::pin(fut)

//             Box::pin(async move {
//                 Ok(req.into_response(
//                     HttpAuthError::InvalidCredentials
//                         .error_response()
//                         .map_into_right_body(),
//                 ))
//             })
//         } else {
//             Box::pin(async move {
//                 Ok(req.into_response(
//                     HttpAuthError::InvalidCredentials
//                         .error_response()
//                         .map_into_right_body(),
//                 ))
//             })
//         }
//     }
// }
