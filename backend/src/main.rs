use std::sync::{Arc, Mutex};

use actix_web::{http::StatusCode, web, App, HttpServer};
use env_logger::Env;
use responses::{Answer, WebError};

mod responses;

#[derive(Clone, Debug)]
struct TodoItem {
    id: u64,
    title: String,
    done: bool,
}

fn render_todo_item(item: &TodoItem) -> TodoOut {
    TodoOut {
        id: item.id,
        title: item.title.clone(),
        done: item.done,
    }
}

#[derive(serde::Serialize)]
struct TodoOut {
    id: u64,
    title: String,
    done: bool,
}

#[derive(serde::Deserialize)]
struct TodoIn {
    title: String,
}

#[derive(Debug, Clone)]
struct TodoState {
    todos: Vec<TodoItem>,
    next_id: u64,
}

#[derive(Debug, Clone)]
struct AppState(Arc<Mutex<TodoState>>);

#[actix_web::get("/todos")]
async fn list_todos(data: web::Data<AppState>) -> Answer<Vec<TodoOut>> {
    let todo_state = Arc::clone(&data.0);
    let todo_state = &todo_state.lock().unwrap();
    Answer::Ok(todo_state.todos.iter().map(render_todo_item).collect())
}

#[actix_web::post("/todos")]
async fn create_todo(data: web::Data<AppState>, todo: web::Json<TodoIn>) -> Answer<u64> {
    let todo_state = Arc::clone(&data.0);
    let mut todo_state = todo_state.lock().unwrap();
    let new_id = todo_state.next_id;
    todo_state.next_id += 1;
    todo_state.todos.push(TodoItem {
        id: new_id,
        title: todo.0.title,
        done: false,
    });

    Answer::Ok(new_id)
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
    path: web::Path<u64>,
) -> Answer<(), TodoItemNotFound> {
    let todo_id = path.into_inner();
    let todo_state = Arc::clone(&data.0);
    let mut todos = todo_state.lock().unwrap();

    if let Some(item) = todos.todos.iter_mut().find(|item| item.id == todo_id) {
        item.done = true;
        Answer::Ok(())
    } else {
        Answer::Err(TodoItemNotFound)
    }
}

#[actix_web::post("/todos/{id}/unmark")]
async fn unmark_todo(
    data: web::Data<AppState>,
    path: web::Path<u64>,
) -> Answer<(), TodoItemNotFound> {
    let todo_id = path.into_inner();
    let todo_state = Arc::clone(&data.0);
    let mut todos = todo_state.lock().unwrap();

    if let Some(item) = todos.todos.iter_mut().find(|item| item.id == todo_id) {
        item.done = false;
        Answer::Ok(())
    } else {
        Answer::Err(TodoItemNotFound)
    }
}

fn initial_todo_state() -> TodoState {
    TodoState {
        todos: vec![
            TodoItem {
                id: 1,
                title: "Procrastinate".into(),
                done: true,
            },
            TodoItem {
                id: 2,
                title: "Learn React".into(),
                done: false,
            },
            TodoItem {
                id: 3,
                title: "Learn Rust".into(),
                done: false,
            },
        ],
        next_id: 4,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let data = web::Data::new(AppState(Arc::new(Mutex::new(initial_todo_state()))));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(list_todos)
            .service(create_todo)
            .service(mark_todo)
            .service(unmark_todo)
    })
    .workers(4)
    .bind(("127.0.0.1", 8123))?
    .run()
    .await
}
