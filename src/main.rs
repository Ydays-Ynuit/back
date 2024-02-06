use actix_web::{App, HttpServer};
use std::env;

mod routes;
mod handlers;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info"); // Vous pouvez régler le niveau de log (error, warn, info, debug, trace)
    env_logger::init();

    log::info!("Démarrage du serveur...");

    HttpServer::new(|| {
        App::new()
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .workers(4)
    .run()
    .await
}
