use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use lambda_http::{run, tracing, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

mod utils;
mod todos;

#[derive(Deserialize, Serialize)]
pub struct Params {
    first: Option<String>,
    second: Option<String>,
}

async fn root() -> Json<Value> {
    Json(json!({ "msg": "I am GET /" }))
}

async fn get_foo() -> Json<Value> {
    Json(json!({ "msg": "I am GET /foo" }))
}

async fn post_foo() -> Json<Value> {
    Json(json!({ "msg": "I am POST /foo" }))
}

async fn post_foo_name(Path(name): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("I am POST /foo/{{name}}, name={name}") }))
}

async fn get_parameters(Query(params): Query<Params>) -> Json<Value> {
    Json(json!({ "request parameters": params }))
}

/// Example on how to return status codes and data from an Axum function
async fn health_check() -> (StatusCode, String) {
    let health = true;
    match health {
        true => (StatusCode::OK, "Healthy!".to_string()),
        false => (StatusCode::INTERNAL_SERVER_ERROR, "Not healthy!".to_string()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let _ = dotenvy::dotenv();
    env_logger::init(); // useless for now

    let db_pool = PgPool::connect(&get_db_url())
        .await
        .expect("Connection to database should not fail");

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/{name}", post(post_foo_name))
        .route("/parameters", get(get_parameters))
        .route("/health", get(health_check))
        .nest("/api/todos", todos::get_todos_router())
        .with_state(db_pool);
    
    run(app).await
}

fn get_db_url() -> String {
    match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            let postgres_user = env_var_with_default_on_empty("POSTGRES_USR", "postgres".to_string());
            let postgres_password = std::env::var("POSTGRES_PWD").expect("A non-default password should be set with `POSTGRES_PWD` env var");
            let postgres_endpoint = env_var_with_default_on_empty("POSTGRES_EDP", "localhost".to_string());
            let postgres_port = env_var_with_default_on_empty("POSTGRES_PRT", "5432".to_string());
            let postgres_databasename = env_var_with_default_on_empty("POSTGRES_DBN", "postgres".to_string());
            let postgres_certificate = std::env::var("POSTGRES_CRT").unwrap_or("".to_string());

            if postgres_certificate.is_empty() {
                format!("postgres://{postgres_user}:{postgres_password}@{postgres_endpoint}:{postgres_port}/{postgres_databasename}")
            } else {
                format!("postgres://{postgres_user}:{postgres_password}@{postgres_endpoint}:{postgres_port}/{postgres_databasename}?sslmode=verify-full&sslrootcert={postgres_certificate}")
            }
        }
    }
}

fn env_var_with_default_on_empty(key: &str, default: String) -> String {
    match std::env::var(key) {
        Ok(var) => if var.is_empty() { default } else { var },
        Err(_) => default,
    }
}
