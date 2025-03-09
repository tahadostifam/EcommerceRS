// use actix_web::{App, HttpServer};

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let cfg: &mut Config = bootstrap_services().cfg.get_mut();
//     let user_service = bootstrap_services().user_service.get_mut();

//     HttpServer::new(|| App::new().service(new_user_controller()))
//         .bind(format!("{}:{}", cfg.server.host, cfg.server.port))
//         .unwrap()
//         .run()
//         .await
// }

fn main() {
    let mut services = ecommercers::bootstrap::bootstrap_services();
    let cfg = services.cfg.borrow_mut();
    let user_service = services.user_service.borrow_mut();

    unimplemented!();
}