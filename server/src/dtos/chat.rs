use serde::{Serialize, Deserialize};
use sqlx::types::chrono::{NaiveDateTime};

#[derive(Deserialize)]
pub struct CreateChatRequest {
    pub user1_email: String,
    pub user2_email: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: i32,
    pub user1_id: i32,
    pub user2_id: i32,
    pub user1_email: Option<String>,
    pub user2_email: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}


#[derive(Serialize)]
pub struct Chat {
    pub id: i32,
    pub user1_id: i32,
    pub user2_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct ChatWithUsers {
    pub id: i32,
    pub user1_email: String,
    pub user2_email: String,
    pub created_at: NaiveDateTime,
}


#[derive(Serialize)]
pub struct MessageResponse {
    pub id: i32,
    pub chat_id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub file_path: Option<String>,
    pub message_type: Option<String>
}