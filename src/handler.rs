use crate::{
    model::{AppState, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
};

use chrono::prelude::*;

use rocket::{
    delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json, State,
};

use uuid::Uuid;

#[get("/health")]
pub async fn health_check_handler() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}

#[get("/todos?<page>&<limit>")]
pub async fn todos_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<TodoListResponse>, Status> {
    let vec = data.todo_db.lock().unwrap();

    // limit to 10
    let limit = limit.unwrap_or(10);
    // pagination
    let offset = (page.unwrap.or(1) - 1) * limit;

    let todos: Vec<Todo> = vec.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };

    Ok(Json(json_response))
}
