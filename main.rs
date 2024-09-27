use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

async fn index() -> &'static str {
    "Hello! This is the main page."
}

async fn projects() -> &'static str {
    "Here are the projects."
}

async fn tasks() -> &'static str {
    "Here are the tasks for a given project."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/projects", web::get().to(projects))
            .route("/tasks", web::get().to(tasks))
    })
    .bind(&server_url)?
    .run()
    .await
}