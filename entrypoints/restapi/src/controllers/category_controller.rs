use crate::{
    dto::category_dto::{
        CategoryCreateDTO, CategoryCreatedDTO, CategoryDeleteDTO, CategoryGetDTO, CategoryUpdateDTO,
    },
    errors::{SimpleMessage, category_errors::HttpCategoryError},
};
use actix_web::{
    HttpResponse, Responder, Scope, get,
    http::{StatusCode, header::ContentType},
    post, web,
};
use ecommercers::core::services::category_service::CategoryService;
use std::sync::{Arc, Mutex};

pub fn new_category_controller() -> Scope {
    web::scope("/categories")
        .service(create_action)
        .service(get_all_action)
        .service(get_action)
        .service(create_action)
        .service(delete_action)
        .service(update_action)
}

#[get("/get")]
async fn get_action(
    category_service_guard: web::Data<Arc<Mutex<CategoryService>>>,
    data: web::Json<CategoryGetDTO>,
) -> Result<impl Responder, HttpCategoryError> {
    let mut category_service = category_service_guard.lock().unwrap();
    let category = category_service.get(data.0.category_id)?;
    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::json())
        .body(serde_json::to_string(&category).unwrap()))
}

#[get("/get_all")]
async fn get_all_action(
    category_service_guard: web::Data<Arc<Mutex<CategoryService>>>,
) -> Result<impl Responder, HttpCategoryError> {
    let mut category_service = category_service_guard.lock().unwrap();
    let categories = category_service.get_all()?;
    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::json())
        .body(serde_json::to_string(&categories).unwrap()))
}

#[post("/create")]
async fn create_action(
    category_service_guard: web::Data<Arc<Mutex<CategoryService>>>,
    data: web::Json<CategoryCreateDTO>,
) -> Result<impl Responder, HttpCategoryError> {
    let mut category_service = category_service_guard.lock().unwrap();
    let category = category_service.create(data.0.name, data.0.description, data.0.parent_id)?;
    Ok(HttpResponse::build(StatusCode::CREATED)
        .insert_header(ContentType::json())
        .body(
            serde_json::to_string(&CategoryCreatedDTO {
                category,
                message: "category created".to_string(),
            })
            .unwrap(),
        ))
}

#[post("/update")]
async fn update_action(
    category_service_guard: web::Data<Arc<Mutex<CategoryService>>>,
    data: web::Json<CategoryUpdateDTO>,
) -> Result<impl Responder, HttpCategoryError> {
    let mut category_service = category_service_guard.lock().unwrap();
    let category = category_service.update(
        data.0.category_id,
        data.0.name,
        data.0.description,
        data.0.parent_id,
    )?;
    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::json())
        .body(
            serde_json::to_string(&CategoryCreatedDTO {
                category,
                message: "category created".to_string(),
            })
            .unwrap(),
        ))
}

#[post("/delete")]
async fn delete_action(
    category_service_guard: web::Data<Arc<Mutex<CategoryService>>>,
    data: web::Json<CategoryDeleteDTO>,
) -> Result<impl Responder, HttpCategoryError> {
    let mut category_service = category_service_guard.lock().unwrap();
    category_service.delete(data.0.category_id)?;
    Ok(HttpResponse::build(StatusCode::OK)
        .insert_header(ContentType::json())
        .body(
            serde_json::to_string(&SimpleMessage {
                message: "category deleted".to_string(),
            })
            .unwrap(),
        ))
}
