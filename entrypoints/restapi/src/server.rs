use actix_web::{App, HttpServer, web};
use controllers::{product_controller::new_product_controller, user_controller::new_user_controller};

mod controllers;
mod errors;
mod dto;
mod middlewares;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let services = ecommercers::bootstrap::bootstrap_services();
    let cfg = services.cfg.lock().unwrap();

    let endpoint_addr = format!("{}:{}", cfg.server.host, cfg.server.port);
    println!("# RestAPI Endpoint: {}", endpoint_addr.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(services.user_service.clone()))
            .app_data(web::Data::new(services.product_service.clone()))
            .service(new_user_controller())
            .service(new_product_controller())
    })
    .bind(endpoint_addr)
    .unwrap()
    .run()
    .await
}
