use actix_web::{
    HttpResponse, Responder, Scope,
    http::{StatusCode, header::ContentType},
    post, web,
};
use ecommercers::core::services::product_service::ProductService;
use std::sync::{Arc, Mutex};

use crate::{
    dto::product_dto::{ProductCreateDTO, ProductCreatedDTO},
    errors::product_errors::HttpProductError,
};

pub fn new_product_controller() -> Scope {
    web::scope("/products").service(create_action)
}

#[post("/create")]
async fn create_action(
    product_service_guard: web::Data<Arc<Mutex<ProductService>>>,
    data: web::Json<ProductCreateDTO>,
) -> Result<impl Responder, HttpProductError> {
    let mut product_service = product_service_guard.lock().unwrap();
    let product = product_service.create(
        data.0.name,
        data.0.description,
        data.0.price,
        data.0.stock,
        data.0.product_image,
    )?;

    Ok(HttpResponse::build(StatusCode::CREATED)
        .insert_header(ContentType::json())
        .body(
            serde_json::to_string(&ProductCreatedDTO {
                product,
                message: "product created".to_string(),
            })
            .unwrap(),
        ))
}
