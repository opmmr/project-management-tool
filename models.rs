use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

fn load_env() {
    dotenv().ok();
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
    OnHold,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub status: Status,
    pub deadline: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub tasks: Vec<Task>,
    pub status: Status,
    pub deadline: DateTime<Utc>,
}

fn main() {
    load_env();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", db_url);
}