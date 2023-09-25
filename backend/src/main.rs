use std::time::Duration;

use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::ErrorResponse,
    routing::{delete, get, post},
    Router,
};
use responses::{Answer, WebError};
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::trace::TraceLayer;

mod db;
mod responses;

#[derive(serde::Serialize)]
struct TodoOut {
    id: i64,
    title: String,
    done: bool,
}

#[derive(serde::Deserialize)]
struct TodoIn {
    title: String,
}

#[derive(Debug, Clone)]
struct AppState {
    pool: sqlx::SqlitePool,
}

#[axum::debug_handler]
async fn list_todos(State(state): State<AppState>) -> axum::response::Result<Answer<Vec<TodoOut>>> {
    let items = db::list_todos(&state.pool)
        .await
        .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Answer::Ok(
        items
            .into_iter()
            .map(|row| TodoOut {
                id: row.id,
                title: row.title,
                done: row.done,
            })
            .collect(),
    ))
}

async fn create_todo(
    State(data): State<AppState>,
    Json(todo): Json<TodoIn>,
) -> axum::response::Result<Answer<i64>> {
    let created_id = db::create_todo(&data.pool, todo.title, false)
        .await
        .map_err(|_| ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Answer::Ok(created_id))
}

pub struct TodoItemNotFound;

impl WebError for TodoItemNotFound {
    fn status(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn code(&self) -> &'static str {
        "todo_item_not_found"
    }
}

async fn mark_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> axum::response::Result<Answer<(), TodoItemNotFound>> {
    match db::mark_todo(&state.pool, id).await {
        Ok(()) => Ok(Answer::Ok(())),
        Err(db::UpdateTodoError::NotFound) => Ok(Answer::Err(TodoItemNotFound)),
        Err(db::UpdateTodoError::SqlError(_)) => {
            Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

async fn unmark_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> axum::response::Result<Answer<(), TodoItemNotFound>> {
    match db::unmark_todo(&state.pool, id).await {
        Ok(()) => Ok(Answer::Ok(())),
        Err(db::UpdateTodoError::NotFound) => Ok(Answer::Err(TodoItemNotFound)),
        Err(db::UpdateTodoError::SqlError(_)) => {
            Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> axum::response::Result<Answer<(), TodoItemNotFound>> {
    match db::delete_todo(&state.pool, id).await {
        Ok(()) => Ok(Answer::Ok(())),
        Err(db::UpdateTodoError::NotFound) => Ok(Answer::Err(TodoItemNotFound)),
        Err(db::UpdateTodoError::SqlError(_)) => {
            Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

async fn slow_middleware<B>(
    req: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> axum::response::Response {
    tokio::time::sleep(Duration::from_millis(300)).await;
    next.run(req).await
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pool = SqlitePoolOptions::new()
        .connect(&std::env::var("DATABASE_URL").expect("Set the DATABASE_URL environment variable"))
        .await
        .unwrap();

    let app = Router::new()
        .route("/todos", get(list_todos))
        .route("/todos", post(create_todo))
        .route("/todos/:id/mark", post(mark_todo))
        .route("/todos/:id/unmark", post(unmark_todo))
        .route("/todos/:id", delete(delete_todo))
        .with_state(AppState { pool })
        .layer(axum::middleware::from_fn(slow_middleware))
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"127.0.0.1:8123".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
