use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

struct AppConfig {
    database_url: String,
}

impl AppConfig {
    pub fn load_from_environment() -> Self {
        dotenv().ok(); // Load .env file if it exists
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        AppConfig { database_url }
    }
}

#[derive(Serialize, Deserialize)]
struct ProjectItem {
    id: u32,
    name: String,
    description: String,
}

mod database {
    use super::*;

    pub fn fetch_item_by_id(item_id: u32) -> Option<ProjectItem> {
        Some(ProjectItem {
            id: item_id,
            name: "Sample Project Item".to_string(),
            description: "This is a sample description for the project item.".to_string(),
        })
    }

    pub fn insert_new_item(new_item: ProjectItem) -> Result<ProjectItem, &'static str> {
        Ok(new_item) // Simulating item creation in a database
    }
}

async fn fetch_item_handler(path: web::Path<(u32,)>) -> impl Responder {
    let item_id = path.into_inner().0;
    match database::fetch_item_by_id(item_id) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().body("Project item not found"),
    }
}

async fn create_item_handler(new_item: web::Json<ProjectItem>) -> impl Responder {
    match database::insert_new_item(new_item.into_inner()) {
        Ok(created_item) => HttpResponse::Created().json(created_item),
        Err(error_message) => HttpResponse::BadRequest().body(error_message),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = AppConfig::load_from_environment();

    println!("Starting server with Database URL: {}", app_config.database_url);

    HttpServer::new(|| {
        App::new()
            .route("/project/item/{id}", web::get().to(fetch_item_handler))
            .route("/project/item", web::post().to(create_item_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}