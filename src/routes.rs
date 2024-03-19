use crate::handlers;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/test", web::get().to(handlers::test_get_handler))
            .route("/test", web::post().to(handlers::test_post_handler))
            .route("/register", web::get().to(handlers::register_handler)),
    );
}
