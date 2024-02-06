use actix_web::{Responder, HttpResponse};
use log::info;
use std::thread;

pub async fn test_get_handler() -> impl Responder {
    let worker_id = thread::current().id();
    info!("Worker {:?} traite une requête GET /api/test", worker_id);
    HttpResponse::Ok().body("Réponse à la requête GET sur /api/test")
}

pub async fn test_post_handler() -> impl Responder {
    let worker_id = thread::current().id();
    info!("Worker {:?} traite une requête POST /api/test", worker_id);
    HttpResponse::Ok().body("Réponse à la requête POST sur /api/test")
}
