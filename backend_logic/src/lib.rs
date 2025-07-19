// #[derive(Deserialize)]
// struct EmailRequest {
//     subject: String,
//     body: String,
// }

// #[derive(Serialize)]
// struct EmailResponse {
//     message: String,
// }

// async fn send_email(event: LambdaEvent<EmailRequest>) -> Result<EmailResponse, Error> {
//     dotenv().ok();

//     let gmail_user = env::var("GMAIL_USR")?;
//     let gmail_pass = env::var("GMAIL_PWD_AWS")?;

//     let email = Message::builder()
//         .from(gmail_user.parse()?)
//         .to(gmail_user.parse()?)
//         .subject(event.payload.subject)
//         .body(event.payload.body)?;

//     let creds = Credentials::new(gmail_user, gmail_pass);

//     let mailer = SmtpTransport::relay("smtp.gmail.com")?
//         .credentials(creds)
//         .build();

//     mailer.send(&email)?;

//     Ok(EmailResponse {
//         message: "Email sent successfully!".to_string(),
//     })
// }
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize)]
pub struct Params {
    first: Option<String>,
    second: Option<String>,
}

pub async fn root() -> Json<Value> {
    Json(json!({ "msg": "I am GET /" }))
}

pub async fn get_foo() -> Json<Value> {
    Json(json!({ "msg": "I am GET /foo" }))
}

pub async fn post_foo() -> Json<Value> {
    Json(json!({ "msg": "I am POST /foo" }))
}

pub async fn post_foo_name(Path(name): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("I am POST /foo/:name, name={name}") }))
}

pub async fn get_parameters(Query(params): Query<Params>) -> Json<Value> {
    Json(json!({ "request parameters": params }))
}

/// Example on how to return status codes and data from an Axum function
pub async fn health_check() -> (StatusCode, String) {
    let health = true;
    match health {
        true => (StatusCode::OK, "Healthy!".to_string()),
        false => (StatusCode::INTERNAL_SERVER_ERROR, "Not healthy!".to_string()),
    }
}
