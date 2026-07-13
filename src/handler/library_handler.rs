use axum::{Json, http::StatusCode};

pub async fn create_library(Json(_): Json<()>) -> (StatusCode, Json<()>) {
    todo!()
}

pub async fn list_libraries(Json(_): Json<()>) -> (StatusCode, Json<()>) {
    todo!()
}
