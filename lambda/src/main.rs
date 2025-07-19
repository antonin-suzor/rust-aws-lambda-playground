use lambda_http::{run, tracing, Error};
use backend_logic::get_axum_app;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(get_axum_app()).await
}