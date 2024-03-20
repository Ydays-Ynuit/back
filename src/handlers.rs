use crate::db::DbPool;
use actix_web::{web, HttpResponse, Responder};
use log::info;
use std::thread; // Import RegisterData correctly

use crate::services::user_service;
use crate::users::models::LoginData;
use crate::users::models::RegisterData;

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

pub async fn register_handler(
    pool: web::Data<DbPool>,
    form: web::Json<RegisterData>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let register_data = form.into_inner();

    match user_service::create_user(&mut *conn, register_data) {
        Ok(new_user) => HttpResponse::Ok().json(new_user), // Retourner l'utilisateur créé en JSON
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
pub async fn login_handler(pool: web::Data<DbPool>, form: web::Json<LoginData>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let login_data = form.into_inner();

    match user_service::login_user(&mut *conn, login_data) {
        Ok((token, user)) => {
            let login_response = serde_json::json!({
                "token": token,
                "user": user
            });
            HttpResponse::Ok().json(login_response) // Retourner le token et l'utilisateur en JSON
        }
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
