use sqlx::PgPool;
use std::sync::{Arc, Mutex};
use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use serde::{Deserialize, Serialize};
use base64::Engine;
use std::fs::File;
use std::io::{Write};


type Connections = Arc<Mutex<Vec<mpsc::UnboundedSender<Message>>>>;

use serde::de::{self, Deserializer, Unexpected};

fn deserialize_str_or_number<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        serde_json::Value::Number(n) if n.is_i64() => {
            n.as_i64().map(|v| v as i32).ok_or_else(|| de::Error::invalid_type(Unexpected::Other("i64"), &"i32"))
        }
        serde_json::Value::String(s) => s.parse::<i32>().map_err(de::Error::custom),
        _ => Err(de::Error::invalid_type(Unexpected::Other("string or number"), &"i32")),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NewMessageRequest {
    #[serde(deserialize_with = "deserialize_str_or_number")]
    chat_id: i32,  // Число, яке може бути передано як рядок чи число
    #[serde(deserialize_with = "deserialize_str_or_number")]
    user_id: i32,  // Число, яке може бути передано як рядок чи число
    content: String, // Рядок
    file_data: Option<String>,
    file_path: Option<String>,
    message_type: String,      // Type of message (e.g., "file" or "text")
}


pub async fn websocket_server(pool: PgPool) {
    let addr = "127.0.0.1:9000";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind WebSocket server");
    let connections = Arc::new(Mutex::new(Vec::new()));

    println!("WebSocket server running on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let connections = connections.clone();
        let pool = pool.clone();

        tokio::spawn(async move {
            if let Ok(ws_stream) = accept_async(stream).await {
                handle_connection(ws_stream, connections, pool).await;
            }
        });
    }
}

async fn handle_connection(
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    connections: Connections,
    pool: PgPool,
) {
    let (mut write, mut read) = ws_stream.split();

    let (tx, mut rx) = mpsc::unbounded_channel();
    connections.lock().unwrap().push(tx);

    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if msg.is_text() {
                let msg_text = msg.to_text().unwrap();
                println!("Received message: {}", msg_text); // Логування отриманого повідомлення

                if let Ok(request) = serde_json::from_str::<NewMessageRequest>(msg_text) {
                    println!("Parsed message: {:?}", request); // Логування розпарсеного повідомлення

                    if request.message_type == "file" {
                        if let Some(ref file_data) = request.file_data {
                            // Використовуємо переданий шлях до файлу
                            if let Some(ref file_path) = request.file_path {
                                if let Err(e) = save_file(&file_data, &file_path).await {
                                    eprintln!("Failed to save file: {}", e);
                                }
                            } else {
                                eprintln!("File path not provided");
                            }
                        }
                    }

                    if let Err(e) = save_message_to_db(&pool, &request).await {
                        eprintln!("Failed to save message: {}", e);
                    }

                    // Формуємо JSON для відповіді
                    let response = serde_json::json!(request);
                    let response_text = response.to_string();

                    // Відправляємо повідомлення всім підключеним користувачам
                    for sender in connections.lock().unwrap().iter() {
                        let _ = sender.send(Message::Text(response_text.clone()));
                    }
                } else {
                    eprintln!("Failed to parse message");
                }
            }
        }
    });

    while let Some(msg) = rx.recv().await {
        let _ = write.send(msg).await;
    }
}

async fn save_message_to_db(
    pool: &PgPool,
    request: &NewMessageRequest,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO messages (chat_id, user_id, content, created_at, file_path, message_type)
        VALUES ($1, $2, $3, NOW(), $4, $5)
        "#,
        request.chat_id,
        request.user_id,
        request.content,
        request.file_path.as_deref(), // Якщо є шлях до файлу, зберігаємо його
        request.message_type
    )
        .execute(pool)
        .await;

    match result {
        Ok(_) => {
            println!("Message saved successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to save message: {}", e);
            Err(e)
        }
    }
}

async fn save_file(file_data: &str, file_path: &str) -> Result<(), std::io::Error> {
    // Видаляємо префікс 'data:*/*;base64,' якщо він є
    let file_content = file_data.split(',').nth(1).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file data"))?;

    // Декодуємо Base64 в бінарний масив
    let decoded_data = base64::prelude::BASE64_STANDARD.decode(file_content)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to decode base64"))?;

    // Створюємо файл та записуємо дані за вказаним шляхом
    let mut file = File::create(file_path)?;
    file.write_all(&decoded_data)?;

    println!("File saved: {}", file_path);

    Ok(())
}
