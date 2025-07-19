use backend_logic::*;
use axum::{
    routing::{get, post},
    Router,
    serve,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/{name}", post(post_foo_name))
        .route("/parameters", get(get_parameters))
        .route("/health/", get(health_check));
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    serve(listener, app)
        .await
        .unwrap();
}
