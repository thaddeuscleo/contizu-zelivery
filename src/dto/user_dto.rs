use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
}

#[derive(Serialize)]
pub struct UserReadDto {
    pub id: u64,
    pub username: String,
}
