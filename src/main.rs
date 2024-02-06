use actix_web::{web, App, HttpServer};
use std::env;

mod routes;
mod handlers;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info"); //(error, warn, info, debug, trace)
    env_logger::init();

    log::info!("Start server...");

    HttpServer::new(|| {
        App::new()
            .configure(routes::init_routes)
            .default_service(
                web::route().to(handlers::not_found)
            )
    })
    .bind("127.0.0.1:8080")?
    .workers(10)
    .run()
    .await
}
