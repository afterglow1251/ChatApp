#[macro_use]
extern crate rocket;

mod routes;
mod dtos;
mod constants;
mod environment;
mod utils;
mod guards;
mod websockets;

use dotenv::dotenv;
use rocket::{Rocket, Build};
use rocket::serde::json::Json;
use rocket::catchers;
use rocket_cors::{AllowedOrigins, CorsOptions};
use sqlx::PgPool;
use crate::environment::{Env};
use routes::auth::{register, login, logout};
use crate::dtos::responses::MessageOnlyResponse;
use crate::routes::chats::{create_chat, get_chat_by_id, get_chat_messages, get_chats};
use crate::routes::users::{get_user, get_users};

use crate::websockets::server::websocket_server;
use tokio::task;

use rocket::fs::{NamedFile};
use rocket::{get, routes};
use std::path::{Path};

#[catch(401)]
fn unauthorized() -> Json<MessageOnlyResponse> {
    Json(MessageOnlyResponse {
        message: "User unauthorized!".to_string(),
    })
}



#[get("/<file_name>")]
async fn download_file(file_name: String) -> Option<NamedFile> {
    let file_path = Path::new("./uploads").join(file_name);

    if file_path.exists() {
        NamedFile::open(file_path).await.ok()
    } else {
        None
    }
}


#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();

    let database_url = Env::database_url();
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to the database");

    let allowed_origins = AllowedOrigins::some_exact(&[Env::client_url()]);
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec!["POST", "GET", "OPTIONS", "PUT", "DELETE", "PATCH"]
            .into_iter()
            .map(|method| method.parse().unwrap())
            .collect(),
        allowed_headers: rocket_cors::AllowedHeaders::some(&["Authorization", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .expect("Failed to create CORS");

    task::spawn(websocket_server(pool.clone()));

    rocket::build()
        .manage(pool)
        .attach(cors)
        .mount("/api/users", routes![register, login, logout, get_users, get_user])
        .mount("/api/chats", routes![create_chat, get_chats, get_chat_by_id, get_chat_messages])
        .mount("/api/files", routes![download_file])
        .register("/", catchers![unauthorized])
}
