use axum::{
	extract::rejection::JsonRejection, 
	http::StatusCode,
	response::{IntoResponse, Json}
};
use axum_macros::FromRequest;
use std::error::Error;
use serde_json::json;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct JsonExtract<T>(pub T);

pub struct ApiError {
	pub status: StatusCode,
	pub message: String,
}
impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        let status = match rejection {
            JsonRejection::JsonDataError(_) => StatusCode::BAD_REQUEST,
            JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
            JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            JsonRejection::BytesRejection(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status,
            message: rejection.to_string(),
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(json!({"message": self.message}))).into_response()
    }
}

pub fn map_err_to_500<E: Error>(err: E) -> ApiError {
    ApiError { status: StatusCode::INTERNAL_SERVER_ERROR, message: err.to_string() }
}

pub fn api_error_500() -> ApiError {
    ApiError { status: StatusCode::INTERNAL_SERVER_ERROR, message: "Internal Server Error".to_string() }
}
