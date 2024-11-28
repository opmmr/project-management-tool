use chrono::{DateTime, Utc, Local};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

fn load_env() {
    dotenv().ok();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
    OnHold,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub status: Status,
    pub deadline: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub tasks: Vec<Task>,
    pub status: Status,
    pub deadline: DateTime<Utc>,
}

impl Project {
    // Add a new task to the project
    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    
    // Change the status of a task
    fn change_task_status(&mut self, task_id: i32, new_status: Status) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = new_status;
        }
    }

    // Display project details
    fn display_project(&self) {
        println!("Project ID: {}, Name: {}, Description: {}, Status: {:?}, Deadline: {}", 
        self.id, self.name, self.description, self.status, self.deadline);
        for task in &self.tasks {
            println!("  Task ID: {}, Name: {}, Description: {}, Status: {:?}, Deadline: {}", 
            task.id, task.name, task.description, task.status, task.deadline);
        }
    }
}

fn main() {
    load_env();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", db_url);

    // Example usage
    let now = Utc::now();
    let mut proj = Project {
        id: 1,
        name: "Rust Project".to_string(),
        description: "A new project management tool in Rust".to_string(),
        tasks: vec![],
        status: Status::NotStarted,
        deadline: now,
    };

    let task = Task {
        id: 1,
        name: "Design Database".to_string(),
        description: "Design the initial database schema".to_string(),
        status: Status::NotStarted,
        deadline: now + chrono::Duration::days(7),
    };

    proj.add_task(task);
    proj.change_task_status(1, Status::InProgress);
    proj.display_project();
}