use axum::{debug_handler, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(todo_get));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize)]
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
}

#[debug_handler]
async fn todo_get() -> Json<TodoItem> {
    Json::from(TodoItem {
        id: 1,
        title: "Learn Rust".to_string(),
        completed: false,
    })
}
