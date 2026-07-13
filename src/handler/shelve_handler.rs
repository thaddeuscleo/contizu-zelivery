use axum::{Json, http::StatusCode};

pub async fn create_shelve(Json(_): Json<()>) -> (StatusCode, Json<()>) {
    todo!()
}

pub async fn list_shelves(Json(_): Json<()>) -> (StatusCode, Json<()>) {
    todo!()
}
