use actix_web::{http::StatusCode, web, App, HttpServer};
use env_logger::Env;
use responses::{Answer, WebError};
use sqlx::sqlite::SqlitePoolOptions;

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

#[actix_web::get("/todos")]
async fn list_todos(
    data: web::Data<AppState>,
) -> Result<Answer<Vec<TodoOut>>, Box<dyn std::error::Error>> {
    let items = db::list_todos(&data.pool).await?;
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

#[actix_web::post("/todos")]
async fn create_todo(
    data: web::Data<AppState>,
    todo: web::Json<TodoIn>,
) -> Result<Answer<i64>, Box<dyn std::error::Error>> {
    let created_id = db::create_todo(&data.pool, todo.0.title, false).await?;
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

#[actix_web::post("/todos/{id}/mark")]
async fn mark_todo(
    data: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<Answer<(), TodoItemNotFound>, Box<dyn std::error::Error>> {
    let id = *path;
    match db::mark_todo(&data.pool, id).await {
        Ok(()) => Ok(Answer::Ok(())),
        Err(db::UpdateTodoError::NotFound) => Ok(Answer::Err(TodoItemNotFound)),
        Err(db::UpdateTodoError::SqlError(e)) => Err(e.into()),
    }
}

#[actix_web::post("/todos/{id}/unmark")]
async fn unmark_todo(
    data: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<Answer<(), TodoItemNotFound>, Box<dyn std::error::Error>> {
    let id = *path;
    match db::unmark_todo(&data.pool, id).await {
        Ok(()) => Ok(Answer::Ok(())),
        Err(db::UpdateTodoError::NotFound) => Ok(Answer::Err(TodoItemNotFound)),
        Err(db::UpdateTodoError::SqlError(e)) => Err(e.into()),
    }
}

#[actix_web::delete("/todos/{id}")]
async fn delete_todo(
    data: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<Answer<(), TodoItemNotFound>, Box<dyn std::error::Error>> {
    let id = *path;
    match db::delete_todo(&data.pool, id).await {
        Ok(()) => Ok(Answer::Ok(())),
        Err(db::UpdateTodoError::NotFound) => Ok(Answer::Err(TodoItemNotFound)),
        Err(db::UpdateTodoError::SqlError(e)) => Err(e.into()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePoolOptions::new()
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let data = web::Data::new(AppState { pool });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(list_todos)
            .service(create_todo)
            .service(mark_todo)
            .service(unmark_todo)
            .service(delete_todo)
    })
    .workers(4)
    .bind(("127.0.0.1", 8123))?
    .run()
    .await
}
