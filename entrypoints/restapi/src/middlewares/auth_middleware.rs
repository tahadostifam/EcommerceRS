use crate::errors::user_errors::HttpAuthError;
use actix_web::Error;
use actix_web::HttpMessage;
use actix_web::ResponseError;
use actix_web::body::EitherBody;
use actix_web::body::MessageBody;
use actix_web::dev::Service;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::dev::Transform;
use actix_web::http::header::AUTHORIZATION;
use ecommercers::core::services::user_service::UserService;
use std::cell::RefCell;
use std::future;
use std::future::Ready;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct AuthMiddleware {
    user_service: Arc<Mutex<UserService>>,
}

pub fn new_auth_middleware(user_service: Arc<Mutex<UserService>>) -> AuthMiddleware {
    AuthMiddleware { user_service }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddlewareActix<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(AuthenticationMiddlewareActix {
            service: Rc::new(RefCell::new(service)),
            user_service: self.user_service.clone(),
        }))
    }
}

pub struct AuthenticationMiddlewareActix<S> {
    service: Rc<RefCell<S>>,
    user_service: Arc<Mutex<UserService>>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddlewareActix<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get(AUTHORIZATION).cloned() {
            let service = self.service.clone();
            let access_token = auth_header.to_str().unwrap()[7..].to_string();
            let mut user_service = self.user_service.lock().unwrap();

            let authorized = user_service.authorization(access_token);

            let fut = async move {
                if let Ok(user) = authorized {
                    req.extensions_mut().insert(user);
                    service.call(req).await.map(|res| res.map_into_left_body())
                } else {
                    Ok(req.into_response(
                        HttpAuthError::InvalidCredentials
                            .error_response()
                            .map_into_right_body(),
                    ))
                }
            };

            Box::pin(fut)
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpAuthError::InvalidCredentials
                        .error_response()
                        .map_into_right_body(),
                ))
            })
        }
    }
}
