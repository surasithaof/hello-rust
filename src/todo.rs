use axum::{
    debug_handler,
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use httpdate::HttpDate;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::SystemTime};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoItem {
    id: Option<String>,
    title: String,
    completed: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// NOTE: In a real application, use proper synchronization (e.g., Mutex) for shared state.
// This is just for now, to keep the example simple.
type TodoStore = Arc<Mutex<Vec<TodoItem>>>;

fn init_store() -> TodoStore {
    Arc::new(Mutex::new(Vec::new()))
}

pub fn init_router() -> Router {
    let store = init_store();

    Router::new()
        .route("/todos", get(list_handler))
        .route("/todos", post(create_handler))
        .route("/todos/{id}", get(get_handler))
        .route("/todos/{id}", patch(patch_handler))
        .route("/todos/{id}", delete(delete_handler))
        .with_state(store)
}

#[debug_handler]
async fn list_handler(State(store): State<TodoStore>) -> Result<Json<Vec<TodoItem>>, StatusCode> {
    let items = store.lock().await;
    Ok(Json(items.clone()))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateTodoItem {
    title: String,
    completed: bool,
}

#[debug_handler]
async fn create_handler(
    State(store): State<TodoStore>,
    Json(new_item): Json<CreateTodoItem>,
) -> impl IntoResponse {
    let now = Utc::now();
    let item = TodoItem {
        id: Some(uuid::Uuid::new_v4().to_string()),
        title: new_item.title.clone(),
        completed: new_item.completed,
        created_at: now,
        updated_at: now,
    };

    store.lock().await.push(item.clone());

    let location = format!("/todos/{}", item.id.as_ref().unwrap());
    IntoResponse::into_response((
        StatusCode::CREATED,
        [(header::LOCATION, location)],
        Json(item.clone()),
    ))
}

#[debug_handler]
async fn get_handler(State(store): State<TodoStore>, Path(id): Path<String>) -> impl IntoResponse {
    let items = store.lock().await;
    let item = items.iter().find(|item| item.id.as_ref() == Some(&id));

    if let Some(item) = item {
        // format the SystemTime as HTTP date
        let last_modified: HttpDate = SystemTime::from(item.updated_at).into();
        let etag = SystemTime::from(item.updated_at)
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        IntoResponse::into_response((
            StatusCode::OK,
            [
                (header::LAST_MODIFIED, last_modified.to_string().as_str()),
                (header::ETAG, format!("{etag}").as_str()),
            ],
            Json(item.clone()),
        ))
    } else {
        IntoResponse::into_response(StatusCode::NOT_FOUND)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PatchTodoItem {
    title: Option<String>,
    completed: Option<bool>,
}

#[debug_handler]
async fn patch_handler(
    State(store): State<TodoStore>,
    Path(id): Path<String>,
    Json(update): Json<PatchTodoItem>,
) -> Result<Json<TodoItem>, StatusCode> {
    // TODO: check if header If-Match or If-Unmodified-Since matches

    let mut items = store.lock().await;
    if let Some(item) = items.iter_mut().find(|item| item.id.as_ref() == Some(&id)) {
        if let Some(title) = update.title {
            item.title = title.clone();
        }
        if let Some(completed) = update.completed {
            item.completed = completed;
        }
        Ok(Json(item.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[debug_handler]
async fn delete_handler(
    State(store): State<TodoStore>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let mut items = store.lock().await;
    let initial_len = items.len();
    items.retain(|item| item.id.as_ref() != Some(&id));
    if items.len() < initial_len {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
