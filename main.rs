// src/handlers.rs

pub async fn index() -> &'static str {
    "Hello! This is the main page."
}

pub async fn projects() -> &'static str {
    "Here are the projects."
}

pub async fn tasks() -> &'static str {
    "Here are the tasks for a given project."
}
```

```rust
// src/main.rs

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

mod handlers; // Import the handlers module

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");

    HttpServer::new(|| {
        App::new()
            // Use the handlers from the handlers module
            .route("/", web::get().to(handlers::index))
            .route("/projects", web::get().to(handlers::projects))
            .route("/tasks", web::get().to(handlers::tasks))
    })
    .bind(&server_url)?
    .run()
    .await
}