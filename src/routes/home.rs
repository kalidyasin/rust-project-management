use actix_web::{get, HttpResponse, Responder}; // Importing actix_web

#[get("/home")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().json("Project Management API Home")
}
