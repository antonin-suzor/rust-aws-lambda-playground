use lambda_http::{run, tracing, Error};
use backend_logic::*;
use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/{name}", post(post_foo_name))
        .route("/parameters", get(get_parameters))
        .route("/health/", get(health_check));

    run(app).await

    // run(get_axum_router()).await
}