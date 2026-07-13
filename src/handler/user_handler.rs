use axum::{Json, http::StatusCode};

use crate::dto::user_dto::{CreateUserDto, UserReadDto};

pub async fn create_user(Json(payload): Json<CreateUserDto>) -> (StatusCode, Json<UserReadDto>) {
    let user = UserReadDto {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
