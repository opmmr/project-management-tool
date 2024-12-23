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

struct SharedState {
    projects: Mutex<Vec<Project>>,
}

async fn add_project(project_details: web::Json<Project>, state: web::Data<SharedState>) -> impl Responder {
    let mut projects = state.projects.lock().unwrap();
    projects.push(project_details.into_inner());
    HttpResponse::Ok().json("Project added successfully")
}

async fn fetch_projects(state: web::Data<SharedState>) -> impl Responder {
    let projects = state.projects.lock().unwrap();
    HttpResponse::Ok().json(&*projects)
}

async fn remove_project(project_id: web::Path<u32>, state: web::Data<SharedState>) -> impl Responder {
    let mut projects = state.projects.lock().unwrap();
    let id = project_id.into_inner();
    projects.retain(|project| project.id != id);
    HttpResponse::Ok().json("Project deleted successfully")
}

async fn replace_project(project_id: web::Path<u32>, new_project: web::Json<Project>, state: web::Data<SharedState>) -> impl Responder {
    let mut projects = state.projects.lock().unwrap();
    let id = project_id.into_inner();
    let new_project_data = new_project.into_inner();
    for project in projects.iter_mut() {
        if project.id == id {
            *project = new_project_data;
            return HttpResponse::Ok().json("Project updated successfully");
        }
    }
    HttpResponse::NotFound().body("Project not found")
}

async fn add_task_to_project(state: web::Data<SharedState>, task_details: web::Json<(u32, Task)>) -> impl Responder {
    let (project_id, new_task) = task_details.into_inner();
    let mut projects = state.projects.lock().unwrap();
    if let Some(project) = projects.iter_mut().find(|p| p.id == project_id) {
        project.tasks.push(new_task);
        return HttpResponse::Ok().json("Task added to the project successfully");
    }
    HttpResponse::NotFound().body("Project not found for adding the task")
}

pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/projects", web::post().to(add_project))
            .route("/projects", web::get().to(fetch_projects))
            .route("/projects/{id}", web::delete().to(remove_project))
            .route("/projects/{id}", web::put().to(replace_project))
            .route("/projects/tasks", web::post().to(add_task_to_project)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let application_state = SharedState {
        projects: Mutex::new(vec![]),
    };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(application_state.clone()))
            .configure(setup_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}