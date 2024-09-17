use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello! This is the main page.")
}

async fn projects() -> impl Responder {
    HttpResponse::Ok().body("Here are the projects.")
}

async fn tasks() -> impl Responder {
    HttpResponse::Ok().body("Here are the tasks for a given project.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/projects", web::get().to(projects))
            .route("/tasks", web::get().to(tasks))
    })
    .bind(server_url)?
    .run()
    .await
}