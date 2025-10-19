use axum::{debug_handler, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoItem {
    id: Option<String>,
    title: String,
    completed: bool,
}

struct TodoList {
    items: Vec<TodoItem>,
}

struct AppState {
    todo_list: TodoList,
}

// NOTE: In a real application, use proper synchronization (e.g., Mutex) for shared state.
// This is just for now, to keep the example simple.
static mut STATE: AppState = AppState {
    todo_list: TodoList { items: vec![] },
};

#[debug_handler]
pub async fn todo_list() -> Json<Vec<TodoItem>> {
    unsafe { STATE.todo_list.items.clone().into() }
}

#[debug_handler]
pub async fn todo_post(Json(mut item): Json<TodoItem>) -> Json<TodoItem> {
    item.id = Some(uuid::Uuid::new_v4().to_string());
    unsafe {
        STATE.todo_list.items.push(item.clone());
    }
    Json::from(item)
}
