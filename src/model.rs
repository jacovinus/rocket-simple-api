use chrono::prelude::*;
use serde::{Deserealize, Serialize};
use std::sync::{Arc, Mutex};

#[allow(non_snake case)]
#[derive(Debug, Deserealize, Serialize, Clone)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String, 
    pub content: String, 
    pub completed: Option<bool>,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
}

pub struct AppState {

    pub todo_db: Arc<Mutex<Vec<Todo>>>,

}

impl ApppState {
    pub fn init() -> AppState {
        AppState {
            todo_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserealize)]
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}