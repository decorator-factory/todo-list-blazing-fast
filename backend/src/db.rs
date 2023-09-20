#[derive(Debug)]
pub struct TodoItem {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

struct TodoItemRow {
    id: i64,
    title: String,
    done: i64,
}

fn parse_row(row: TodoItemRow) -> TodoItem {
    TodoItem {
        id: row.id,
        title: row.title,
        done: match row.done {
            0 => false,
            1 => true,
            n => panic!("Unexpected integer {n} in boolean row `todo_items.done`"),
        },
    }
}

pub async fn list_todos(pool: &sqlx::SqlitePool) -> sqlx::Result<Vec<TodoItem>> {
    let rows = sqlx::query_as!(
        TodoItemRow,
        "SELECT id, title, done FROM todo_items ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(parse_row).collect())
}

pub async fn create_todo(
    pool: &sqlx::SqlitePool,
    title: String,
    done: bool,
) -> Result<i64, sqlx::Error> {
    let done = i32::from(done);
    let created_id = sqlx::query!(
        "INSERT INTO todo_items (title, done) VALUES (?, ?) RETURNING id;",
        title,
        done,
    )
    .fetch_one(pool)
    .await?
    .id;

    Ok(created_id)
}

pub enum UpdateTodoError {
    NotFound,
    SqlError(sqlx::Error),
}

pub async fn delete_todo(pool: &sqlx::SqlitePool, id: i64) -> Result<(), UpdateTodoError> {
    sqlx::query!("DELETE FROM todo_items WHERE id = ? RETURNING id;", id)
        .fetch_optional(pool)
        .await
        .map_err(UpdateTodoError::SqlError)?
        .map(|_| ())
        .ok_or(UpdateTodoError::NotFound)
}

pub async fn mark_todo(pool: &sqlx::SqlitePool, id: i64) -> Result<(), UpdateTodoError> {
    sqlx::query!(
        "UPDATE todo_items SET done = 1 WHERE id = ? RETURNING id;",
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(UpdateTodoError::SqlError)?
    .map(|_| ())
    .ok_or(UpdateTodoError::NotFound)
}

pub async fn unmark_todo(pool: &sqlx::SqlitePool, id: i64) -> Result<(), UpdateTodoError> {
    sqlx::query!(
        "UPDATE todo_items SET done = 0 WHERE id = ? RETURNING id;",
        id
    )
    .fetch_optional(pool)
    .await
    .map_err(UpdateTodoError::SqlError)?
    .map(|_| ())
    .ok_or(UpdateTodoError::NotFound)
}
