use crate::db::init_pool;
use actix_web::{web, App, HttpServer};
use std::env;

mod db;
mod handlers;
mod routes;
mod schema;

mod services;
mod users;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info"); //(error, warn, info, debug, trace)
    env_logger::init();
    dotenvy::dotenv().ok(); // Load .env file

    log::info!("Start server...");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_pool(&database_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Ajoutez cette ligne
            .configure(routes::init_routes)
            .default_service(web::route().to(handlers::not_found))
    })
    .bind("127.0.0.1:8080")?
    .workers(10)
    .run()
    .await
}
