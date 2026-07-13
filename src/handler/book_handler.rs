use axum::{Json, http::StatusCode};

pub async fn create_book(Json(_): Json<()>) -> (StatusCode, Json<()>) {
    todo!()
}

pub async fn list_books(Json(_): Json<()>) -> (StatusCode, Json<()>) {
    todo!()
}
