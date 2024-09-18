use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder}; // Importing actix_web
use env_logger::Env; // Importing env_logger
use serde::{Deserialize, Serialize}; // Importing Serialize and Deserialize traits //

// Define a struct that will represent the data structure for JSON
#[derive(Serialize, Deserialize)]
struct LoginInputs {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct SignupInputs {
    name: String,
    email: String,
    password: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Project Management API")
}

#[post("/login")]
async fn login(data: web::Json<LoginInputs>) -> impl Responder {
    if data.email == "kalidyasin29@gmail.com" && data.password == "123456" {
        HttpResponse::Ok().json(data.into_inner())
    } else {
        HttpResponse::Unauthorized().json("Invalid Email Or Password")
    }
}

#[post("/signup")]
async fn signup(_data: web::Json<SignupInputs>) -> impl Responder {
    HttpResponse::Ok().json("Successfuly Registerd")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize env_logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default()) // Adding logger middleware
            .service(index)
            .service(login)
            .service(signup)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
