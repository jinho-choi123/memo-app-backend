use actix_web::{get, HttpResponse, Responder};

#[get("/checkserver")]
pub async fn checkserver() -> impl Responder {
    HttpResponse::Ok().body("server is running!")
}