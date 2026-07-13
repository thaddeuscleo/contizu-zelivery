use axum::{
    Router,
    routing::{get, post},
};

use crate::handler::{root_handler, user_handler};

mod dto;
mod handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root_handler::root))
        .route("/users", post(user_handler::create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
