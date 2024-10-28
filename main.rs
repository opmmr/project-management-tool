pub async fn get_main_page() -> &'static str {
    "Hello! This is the main page."
}

pub async fn get_projects_overview() -> &'static str {
    "Here are the projects."
}

pub async fn get_project_tasks() -> &'static str {
    "Here are the tasks for a given project."
}
```

```rust
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod handlers; // Ensure the handlers module is available

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env file
    let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");

    HttpServer::new(|| {
        App::new()
            // Routes utilizing renamed handler functions for clarity
            .route("/", web::get().to(handlers::get_main_page))
            .route("/projects", web::get().to(handlers::get_projects_overview))
            .route("/tasks", web::get().to(handlers::get_project_tasks))
    })
    .bind(&server_url)? // Bind server to URL specified in environment
    .run()
    .await
}