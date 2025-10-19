use axum::{
    routing::{get, post},
    Router,
};

use crate::todo::{todo_list, todo_post};

mod todo;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/todos", get(todo_list))
        .route("/todos", post(todo_post));

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);
    axum::serve(listener, router).await.unwrap();
}
