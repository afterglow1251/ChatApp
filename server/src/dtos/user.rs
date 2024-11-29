use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserWithoutPassword {
    pub id: Option<i32>,
    pub email: String,
}
