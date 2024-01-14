use crate::{
    model::{AppState, Todo, UpdateTodoSchema},
    response:: {GenericResponse, SingleTodoResponse, TodoData, TodoListResponse}
};

use chrono::prelude::*;

use rocket:: {
    delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json, State, 
};

use uuid::Uuid; 


#[get("/health")]
pub async fn health_check_handler() -> Result<Json<GenericResponse>, Status> {

    const  MESSAGE: &str = "Build Simple CRUD API with Rust and Rocket";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}