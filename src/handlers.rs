use actix_web::{Responder, HttpResponse};
use log::info;
use std::thread;

pub async fn test_get_handler() -> impl Responder {
    let worker_id = thread::current().id();
    info!("Worker {:?} processes a GET request /api/test", worker_id);
    HttpResponse::Ok().body("Response to GET request on /api/test")
}

pub async fn test_post_handler() -> impl Responder {
    let worker_id = thread::current().id();
    info!("Worker {:?} processes a POST request /api/test", worker_id);
    HttpResponse::Ok().body("Response to POST request on /api/test")
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found")
}
