use actix_web::{
    HttpResponse, Responder, Scope, get,
    http::{StatusCode, header::ContentType},
    post, web,
};
use ecommercers::core::services::product_service::ProductService;
use std::sync::{Arc, Mutex};

use crate::{
    dto::product_dto::{
        ProductCreateDTO, ProductCreatedDTO, ProductDeleteDTO, ProductGetDTO, ProductSearchDTO,
        ProductUpdateDTO, ProductUpdatedDTO,
    },
    errors::{SimpleMessage, product_errors::HttpProductError},
};

pub fn new_product_controller() -> Scope {
    web::scope("/products")
        .service(create_action)
        .service(get_action)
        .service(search_action)
        .service(update_action)
        .service(delete_action)
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

#[get("/get")]
async fn get_action(
    product_service_guard: web::Data<Arc<Mutex<ProductService>>>,
    data: web::Json<ProductGetDTO>,
) -> Result<impl Responder, HttpProductError> {
    let mut product_service = product_service_guard.lock().unwrap();
    let product = product_service.get(data.0.product_id)?;
    Ok(HttpResponse::build(StatusCode::CREATED)
        .insert_header(ContentType::json())
        .body(serde_json::to_string(&product).unwrap()))
}

#[get("/search")]
async fn search_action(
    product_service_guard: web::Data<Arc<Mutex<ProductService>>>,
    data: web::Json<ProductSearchDTO>,
) -> Result<impl Responder, HttpProductError> {
    let mut product_service = product_service_guard.lock().unwrap();
    let products = product_service.search(data.0.search)?;
    Ok(HttpResponse::build(StatusCode::CREATED)
        .insert_header(ContentType::json())
        .body(serde_json::to_string(&products).unwrap()))
}

#[post("/update")]
async fn update_action(
    product_service_guard: web::Data<Arc<Mutex<ProductService>>>,
    data: web::Json<ProductUpdateDTO>,
) -> Result<impl Responder, HttpProductError> {
    let mut product_service = product_service_guard.lock().unwrap();
    let product = product_service.update(
        data.0.product_id,
        data.0.name,
        data.0.description,
        data.0.price,
        data.0.stock,
        data.0.product_image,
    )?;
    Ok(HttpResponse::build(StatusCode::CREATED)
        .insert_header(ContentType::json())
        .body(
            serde_json::to_string(&ProductUpdatedDTO {
                message: "product updated".to_string(),
                product,
            })
            .unwrap(),
        ))
}

#[post("/delete")]
async fn delete_action(
    product_service_guard: web::Data<Arc<Mutex<ProductService>>>,
    data: web::Json<ProductDeleteDTO>,
) -> Result<impl Responder, HttpProductError> {
    let mut product_service = product_service_guard.lock().unwrap();
    product_service.delete(data.0.product_id)?;
    Ok(HttpResponse::build(StatusCode::CREATED)
        .insert_header(ContentType::json())
        .body(
            serde_json::to_string(&SimpleMessage {
                message: "product deleted".to_string(),
            })
            .unwrap(),
        ))
}
