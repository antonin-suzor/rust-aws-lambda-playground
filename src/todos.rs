//! Quick and dirty implementation of a todo list API using Axum and SQLx with PostgreSQL.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, patch},
    Router,
};
use serde::{Deserialize, Serialize};
use std::fmt::Error;
use sqlx::{FromRow, PgPool, Row};

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

type ServerError = (StatusCode, String);
fn internal_server_error<E: std::error::Error>(err: E) -> ServerError {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub async fn get_api_todos(
	State(db_pool): State<PgPool>,
) -> Result<Json<Vec<Todo>>, ServerError> {
	let vec_todos = sqlx::query("SELECT * FROM todos;")
        .fetch_all(&db_pool)
        .await
        .map_err(internal_server_error)?
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
	State(db_pool): State<PgPool>, Json(todo): Json<TodoDTO>,
) -> Result<Json<Todo>, ServerError> {
    let q = sqlx::query("INSERT INTO todos (title, done) VALUES ($1, $2) RETURNING id;")
        .bind(todo.title.as_ref().map_or("", |s| s.as_ref()))
        .bind(todo.done.unwrap_or(false))
        .fetch_one(&db_pool).await
        .map_err(internal_server_error)?;

	Ok(Json(Todo::new(q.get("id"), todo)))
}

pub async fn patch_api_todos_id(
    Path(todo_id): Path<i32>,
	State(db_pool): State<PgPool>, Json(mod_todo): Json<TodoDTO>,
) -> Result<Json<Todo>, ServerError> {
    let mut todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1;")
        .bind(todo_id)
        .fetch_optional(&db_pool)
        .await
        .map_err(internal_server_error)?
		.ok_or(internal_server_error(Error))?;

    todo.receive(mod_todo);
    sqlx::query(
        "UPDATE todos SET title = $1, done = $2 WHERE id = $3;",
    )
    .bind(&todo.title)
    .bind(todo.done)
    .bind(todo_id)
    .execute(&db_pool)
    .await
    .map_err(internal_server_error)?;
    
	Ok(Json(todo))
}

pub async fn delete_api_todos_id(
	Path(todo_id): Path<i32>,
	State(db_pool): State<PgPool>,
) -> Result<(), ServerError> {
	sqlx::query("DELETE FROM todos WHERE id = $1;")
		.bind(todo_id)
		.execute(&db_pool)
		.await
		.map_err(internal_server_error)?;
	
	Ok(())
}

pub fn get_todos_router() -> Router<PgPool> {
	Router::new()
		.route("/", get(get_api_todos).post(post_api_todos))
		.route("/{todo_id}", patch(patch_api_todos_id).delete(delete_api_todos_id))
}
