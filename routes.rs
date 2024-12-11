use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct Project {
    id: u32,
    name: String,
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    name: String,
    completed: bool,
}

struct AppState {
    projects: Mutex<Vec<Project>>,
}

async fn create_project(data: web::Json<Project>, app_data: web::Data<AppState>) -> impl Responder {
    let mut projects = app_data.projects.lock().unwrap();
    projects.push(data.into_inner());
    HttpResponse::Ok().json("Project added successfully")
}

async fn get_projects(app_data: web::Data<AppState>) -> impl Responder {
    let projects = app_data.projects.lock().unwrap();
    HttpResponse::Ok().json(&*projects)
}

async fn delete_project(path: web::Path<u32>, app_data: web::Data<AppState>) -> impl Responder {
    let mut projects = app_data.projects.lock().unwrap();
    let id = path.into_inner();
    projects.retain(|project| project.id != id);
    HttpResponse::Ok().json("Project deleted successfully")
}

async fn update_project(path: web::Path<u32>, project: web::Json<Project>, app_data: web::Data<AppState>) -> impl Responder {
    let mut projects = app_data.projects.lock().unwrap();
    let id = path.into_inner();
    let project = project.into_inner();
    for p in projects.iter_mut() {
        if p.id == id {
            *p = project;
            return HttpResponse::Ok().json("Project updated successfully");
        }
    }
    HttpResponse::NotFound().body("Project not found")
}

async fn modify_tasks(app_data: web::Data<AppState>, req: web::Json<(u32, Task)>) -> impl Responder {
    let (project_id, task) = req.into_inner();
    let mut projects = app_data.projects.lock().unwrap();
    if let Some(project) = projects.iter_mut().find(|p| p.id == project_id) {
        project.tasks.push(task);
        return HttpResponse::Ok().json("Task added to the project successfully");
    }
    HttpResponse::NotFound().body("Project not found for adding the task")
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/projects", web::post().to(create_project))
            .route("/projects", web::get().to(get_projects))
            .route("/projects/{id}", web::delete().to(delete_project))
            .route("/projects/{id}", web::put().to(update_project))
            .route("/projects/tasks", web::post().to(modify_tasks)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState {
        projects: Mutex::new(vec![]),
    };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}