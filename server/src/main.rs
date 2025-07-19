use backend_logic::get_axum_app;
use axum::{
    serve,
};

#[tokio::main]
async fn main() {
    let app = get_axum_app();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    serve(listener, app)
        .await
        .unwrap();
}
