use rocket::State;
use rocket::serde::json::{Json, Value};
use sqlx::PgPool;
use crate::guards::auth::AuthGuard;
use crate::dtos::user::UserWithoutPassword;
use rocket::http::Status;
use rocket::response::status;


#[get("/", format = "json")]
pub async fn get_users(
    _auth: AuthGuard,
    pool: &State<PgPool>,
) -> Result<Json<Vec<UserWithoutPassword>>, status::Custom<Json<Value>>> {
    let users = sqlx::query!(
        "SELECT id, email FROM users"
    )
        .fetch_all(pool.inner())
        .await
        .map_err(|_| {
            let error_message = serde_json::json!({
            "message": "Failed to fetch users"
        });
            status::Custom(Status::InternalServerError, Json(error_message)) // Повертаємо JSON з помилкою
        })?;

    let users_response: Vec<UserWithoutPassword> = users.into_iter().map(|user| {
        UserWithoutPassword {
            id: Some(user.id),
            email: user.email,
        }
    }).collect();

    Ok(Json(users_response))
}

#[get("/<id>", format = "json")]
pub async fn get_user(
    _auth: AuthGuard,
    pool: &State<PgPool>,
    id: i32,
) -> Result<Json<UserWithoutPassword>, status::Custom<Json<Value>>> {
    let user = sqlx::query!(
        "SELECT id, email FROM users WHERE id = $1",
        id
    )
        .fetch_one(pool.inner())
        .await
        .map_err(|_| {
            let error_message = serde_json::json!({
            "message": "User not found"
        });
            status::Custom(Status::NotFound, Json(error_message))
        })?;

    let user_response = UserWithoutPassword {
        id: Some(user.id),
        email: user.email,
    };

    Ok(Json(user_response))
}