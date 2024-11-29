use rocket::State;
use rocket::serde::json::{Json, Value};
use sqlx::PgPool;
use crate::guards::auth::AuthGuard;
use crate::dtos::chat::{CreateChatRequest, ChatResponse, MessageResponse};
use rocket::http::Status;
use rocket::response::status;

#[post("/", format = "json", data = "<chat_request>")]
pub async fn create_chat(
    _auth: AuthGuard,
    pool: &State<PgPool>,
    chat_request: Json<CreateChatRequest>,
) -> Result<Json<ChatResponse>, status::Custom<Json<Value>>> {
    let user1_email = &chat_request.user1_email;
    let user2_email = &chat_request.user2_email;

    // Перевірка: чи однакові email
    if user1_email == user2_email {
        let error_message = serde_json::json!({
            "message": "User emails must be different"
        });
        return Err(status::Custom(Status::BadRequest, Json(error_message)));
    }

    // Отримати ID користувачів за email
    let user1_id = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        user1_email
    )
        .fetch_one(pool.inner())
        .await
        .map_err(|_| {
            let error_message = serde_json::json!({
            "message": format!("User with email '{}' not found", user1_email)
        });
            status::Custom(Status::NotFound, Json(error_message))
        })?
        .id;

    let user2_id = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        user2_email
    )
        .fetch_one(pool.inner())
        .await
        .map_err(|_| {
            let error_message = serde_json::json!({
            "message": format!("User with email '{}' not found", user2_email)
        });
            status::Custom(Status::NotFound, Json(error_message))
        })?
        .id;

    // Перевірка: чи вже існує чат між цими користувачами
    let existing_chat = sqlx::query!(
        r#"
        SELECT id FROM chats
        WHERE (user1_id = $1 AND user2_id = $2) OR (user1_id = $2 AND user2_id = $1)
        "#,
        user1_id,
        user2_id
    )
        .fetch_optional(pool.inner())
        .await
        .map_err(|_| {
            let error_message = serde_json::json!({
            "message": "Failed to check for existing chat"
        });
            status::Custom(Status::InternalServerError, Json(error_message))
        })?;

    if existing_chat.is_some() {
        let error_message = serde_json::json!({
            "message": "Chat between these users already exists"
        });
        return Err(status::Custom(Status::Conflict, Json(error_message)));
    }

    // Створення нового чату, зберігаючи email користувачів
    let chat = sqlx::query!(
        r#"
        INSERT INTO chats (user1_id, user2_id, user1_email, user2_email, created_at)
        VALUES ($1, $2, $3, $4, NOW())
        RETURNING id, user1_id, user2_id, user1_email, user2_email, created_at
        "#,
        user1_id,
        user2_id,
        user1_email,
        user2_email
    )
        .fetch_one(pool.inner())
        .await
        .map_err(|_| {
            let error_message = serde_json::json!({
            "message": "Failed to create chat"
        });
            status::Custom(Status::InternalServerError, Json(error_message))
        })?;

    // Повернення чату у відповіді
    Ok(Json(ChatResponse {
        id: chat.id,
        user1_id: chat.user1_id,
        user2_id: chat.user2_id,
        user1_email: chat.user1_email,
        user2_email: chat.user2_email,
        created_at: chat.created_at,
    }))
}

#[get("/<chat_id>/messages/all")]
pub async fn get_chat_messages(
    pool: &State<PgPool>,
    chat_id: i32,
) -> Result<Json<Vec<MessageResponse>>, status::Custom<Json<Value>>> {
    let messages = sqlx::query!(
        "SELECT id, chat_id, user_id, content, created_at, file_path, message_type FROM messages WHERE chat_id = $1 ORDER BY created_at ASC",
        chat_id
    )
        .fetch_all(pool.inner())
        .await
        .map_err(|_| {
            let error_message = serde_json::json!({ "message": "Failed to fetch messages" });
            status::Custom(Status::InternalServerError, Json(error_message))
        })?;

    let response = messages.into_iter().map(|msg| MessageResponse {
        id: msg.id,
        chat_id: msg.chat_id,
        user_id: msg.user_id,
        content: msg.content,
        created_at: msg.created_at,
        file_path: msg.file_path,
        message_type: msg.message_type
    }).collect::<Vec<MessageResponse>>();

    Ok(Json(response))
}

#[get("/user/<user_id>", format = "json")]
pub async fn get_chats(
    _auth: AuthGuard,
    pool: &State<PgPool>,
    user_id: i32,
) -> Result<Json<Vec<ChatResponse>>, status::Custom<Json<Value>>> {
    // Запит з JOIN для отримання чатів разом з емейлами
    let chats = sqlx::query!(
        r#"
        SELECT
            c.id,
            c.user1_id,
            c.user2_id,
            u1.email AS user1_email,
            u2.email AS user2_email,
            c.created_at
        FROM chats c
        JOIN users u1 ON c.user1_id = u1.id
        JOIN users u2 ON c.user2_id = u2.id
        WHERE c.user1_id = $1 OR c.user2_id = $1
        ORDER BY c.created_at DESC
        "#,
        user_id
    )
        .fetch_all(pool.inner())
        .await
        .map_err(|_| {
            let error_message = serde_json::json!({
            "message": "Failed to fetch chats"
        });
            status::Custom(Status::InternalServerError, Json(error_message))
        })?;

    if chats.is_empty() {
        return Ok(Json(Vec::new()));
    }

    // Формуємо відповіді з чатом і емейлами
    let chat_responses: Vec<ChatResponse> = chats
        .into_iter()
        .map(|chat| ChatResponse {
            id: chat.id,
            user1_id: chat.user1_id,
            user2_id: chat.user2_id,
            user1_email: Some(chat.user1_email),  // емейл користувача 1
            user2_email: Some(chat.user2_email),  // емейл користувача 2
            created_at: chat.created_at,
        })
        .collect();

    Ok(Json(chat_responses))
}


#[get("/<chat_id>", format = "json")]
pub async fn get_chat_by_id(
    _auth: AuthGuard,   // Авторизаційний гвард для перевірки доступу
    pool: &State<PgPool>, // Доступ до пулу з'єднань з базою даних
    chat_id: i32,       // Ідентифікатор чату
) -> Result<Json<ChatResponse>, status::Custom<Json<Value>>> {
    // Запит для отримання чату за його id
    let chat = sqlx::query!(
        r#"
        SELECT
            c.id,
            c.user1_id,
            c.user2_id,
            u1.email AS user1_email,
            u2.email AS user2_email,
            c.created_at
        FROM chats c
        JOIN users u1 ON c.user1_id = u1.id
        JOIN users u2 ON c.user2_id = u2.id
        WHERE c.id = $1
        "#,
        chat_id
    )
        .fetch_optional(pool.inner()) // Використовуємо `fetch_optional` для отримання або відсутності результатів
        .await
        .map_err(|_| {
            let error_message = serde_json::json!({
                "message": "Failed to fetch chat"
            });
            status::Custom(Status::InternalServerError, Json(error_message))
        })?;

    match chat {
        Some(chat) => {
            // Якщо чат знайдений, повертаємо його як відповідь
            Ok(Json(ChatResponse {
                id: chat.id,
                user1_id: chat.user1_id,
                user2_id: chat.user2_id,
                user1_email: Some(chat.user1_email),  // емейл користувача 1
                user2_email: Some(chat.user2_email),  // емейл користувача 2
                created_at: chat.created_at,
            }))
        }
        None => {
            // Якщо чат не знайдений, повертаємо помилку
            let error_message = serde_json::json!({
                "message": format!("Chat with id '{}' not found", chat_id)
            });
            Err(status::Custom(Status::NotFound, Json(error_message)))
        }
    }
}

