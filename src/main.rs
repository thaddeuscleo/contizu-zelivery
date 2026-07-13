use axum::{
    Router,
    routing::{get, post},
};

use crate::handler::{book_handler, library_handler, root_handler, shelve_handler, user_handler};

mod dto;
mod handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root_handler::root))
        .route("/users", post(user_handler::create_user))
        .route("/shelf", get(shelve_handler::list_shelves))
        .route("/shelf", post(shelve_handler::create_shelve))
        .route("/books", get(book_handler::list_books))
        .route("/books", post(book_handler::create_book))
        .route("/libraries", get(library_handler::list_libraries))
        .route("/libraries", post(library_handler::create_library));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
