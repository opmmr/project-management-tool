use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

struct Config {
    db_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); // Load .env file if it exists
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Config { db_url }
    }
}

#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
    description: String,
}

mod db {
    use super::*;

    pub fn get_item(id: u32) -> Option<Item> {
        Some(Item {
            id,
            name: "Sample item".to_string(),
            description: "This is a sample description.".to_string(),
        })
    }

    pub fn create_item(item: Item) -> Result<Item, &'static str> {
        Ok(item)
    }
}

async fn get_item_handler(path: web::Path<(u32,)>) -> impl Responder {
    let item_id = path.into_inner().0;
    match db::get_item(item_id) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().body("Item not found"),
    }
}

async fn create_item_handler(item: web::Json<Item>) -> impl Responder {
    match db::create_item(item.into_inner()) {
        Ok(created_item) => HttpResponse::Created().json(created_item),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();

    println!("Starting server with DB URL: {}", config.db_url);

    HttpServer::new(|| {
        App::new()
            .route("/item/{id}", web::get().to(get_item_handler))
            .route("/item", web::post().to(create_item_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}