//! Quick and dirty implementation of a todo list API using Axum and SQLx with PostgreSQL.

use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, patch},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};

use crate::utils::{api_error_500, map_err_to_500, ApiError, JsonExtract};

#[derive(Deserialize, FromRow, Serialize)]
pub struct Todo {
    id: i32,
    title: String,
    done: bool,
}
impl Todo {
    fn new(id: i32, ed_request: TodoDTO) -> Self {
        Self {
            id,
            title: ed_request.title.unwrap_or(String::from("")),
            done: ed_request.done.unwrap_or(false)
        }
    }
    fn receive(&mut self, changes: TodoDTO) {
        if changes.title.is_some() {
            self.title = changes.title.unwrap();
        }
        if changes.done.is_some() {
            self.done = changes.done.unwrap();
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TodoDTO {
    title: Option<String>,
    done: Option<bool>,
}
impl From<Todo> for TodoDTO {
    fn from(value: Todo) -> Self {
        Self {
            title: Some(value.title),
            done: Some(value.done),
        }
    }
}

pub async fn get_api_todos(
	State(db_pool): State<PgPool>,
) -> Result<Json<Vec<Todo>>, ApiError> {
	let vec_todos = sqlx::query("SELECT * FROM todos;")
        .fetch_all(&db_pool)
        .await
        .map_err(map_err_to_500)?
		.iter().map(|row| {
			let id: i32 = row.get("id");
			let title: String = row.get("title");
			let done: bool = row.get("done");
			Todo { id, title, done }
		})
		.collect::<Vec<Todo>>();
	Ok(Json(vec_todos))
}

pub async fn post_api_todos(
	State(db_pool): State<PgPool>, JsonExtract(todo): JsonExtract<TodoDTO>,
) -> Result<Json<Todo>, ApiError> {
    let q = sqlx::query("INSERT INTO todos (title, done) VALUES ($1, $2) RETURNING id;")
        .bind(todo.title.as_ref().map_or("", |s| s.as_ref()))
        .bind(todo.done.unwrap_or(false))
        .fetch_one(&db_pool).await
        .map_err(map_err_to_500)?;

	Ok(Json(Todo::new(q.get("id"), todo)))
}

pub async fn patch_api_todos_id(
    Path(todo_id): Path<i32>,
	State(db_pool): State<PgPool>, JsonExtract(mod_todo): JsonExtract<TodoDTO>,
) -> Result<Json<Todo>, ApiError> {
    let mut todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1;")
        .bind(todo_id)
        .fetch_optional(&db_pool)
        .await
        .map_err(map_err_to_500)?
		.ok_or(api_error_500())?;

    todo.receive(mod_todo);
    sqlx::query(
        "UPDATE todos SET title = $1, done = $2 WHERE id = $3;",
    )
    .bind(&todo.title)
    .bind(todo.done)
    .bind(todo_id)
    .execute(&db_pool)
    .await
    .map_err(map_err_to_500)?;
    
	Ok(Json(todo))
}

pub async fn delete_api_todos_id(
	Path(todo_id): Path<i32>,
	State(db_pool): State<PgPool>,
) -> Result<(), ApiError> {
	sqlx::query("DELETE FROM todos WHERE id = $1;")
		.bind(todo_id)
		.execute(&db_pool)
		.await
		.map_err(map_err_to_500)?;
	
	Ok(())
}

/// Router for /api/todos
pub fn get_todos_router() -> Router<PgPool> {
	Router::new()
		.route("/", get(get_api_todos).post(post_api_todos))
		.route("/{todo_id}", patch(patch_api_todos_id).delete(delete_api_todos_id))
}
