use rocket::serde::{Deserialize, Serialize};
use crate::dtos::user::UserWithoutPassword;

#[derive(Serialize, Deserialize)]
pub struct MessageOnlyResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: Option<String>,
    pub user: Option<UserWithoutPassword>,
}