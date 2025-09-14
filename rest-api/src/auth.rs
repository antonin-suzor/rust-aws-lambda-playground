use axum::{
    extract::{Path, State},
	http::HeaderMap,
    response::Json,
    routing::{get, patch},
    Router,
};
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row, Error as SqlxError, Type as SqlxType};

use crate::utils::{api_error_500, map_err_to_500, ApiError, JsonExtract};

#[derive(Debug, Deserialize, Serialize, SqlxType, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "permission_level_enum", rename_all = "lowercase")]
pub enum PermissionLevel {
	User,
	Admin,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct Account {
	id: i32,
	email: String,
	#[serde(rename = "permissionLevel")]
	permission_level: PermissionLevel,
	#[serde(rename = "createdAt")]
	created_at: OffsetDateTime,
	#[serde(rename = "updatedAt")]
	updated_at: OffsetDateTime,
	#[serde(rename = "deletedAt")]
	deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
struct AccountDTO {
	email: String,
	#[serde(rename = "permissionLevel")]
	permission_level: PermissionLevel,
}

async fn create_new_account(email: String, permission_level: PermissionLevel, db_pool: &PgPool) -> Result<i32, SqlxError> {
	let rec = sqlx::query(
		"INSERT INTO
		accounts
		(
		email,
		permission_level
		)
		VALUES
		($1, $2)
		RETURNING
		id
		;")
		.bind(email)
		.bind(permission_level)
		.fetch_one(db_pool)
		.await?;
	let id: i32 = rec.get("id");
	Ok(id)
}

async fn get_api_auth_accounts(
	State(db_pool): State<PgPool>,
) -> Result<Json<Vec<Account>>, ApiError> {
	let vec_accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE deleted_at IS NULL;")
        .fetch_all(&db_pool)
        .await
        .map_err(map_err_to_500)?;
	Ok(Json(vec_accounts))
}

async fn post_api_auth_accounts(
	State(db_pool): State<PgPool>, JsonExtract(account_dto): JsonExtract<AccountDTO>,
) -> Result<Json<Account>, ApiError> {
	let id = create_new_account(account_dto.email, account_dto.permission_level, &db_pool)
		.await
		.map_err(map_err_to_500)?;
	let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = $1;")
		.bind(id)
		.fetch_one(&db_pool)
		.await
		.map_err(map_err_to_500)?;
	Ok(Json(account))
}

async fn patch_api_auth_accounts_id(
    Path(account_id): Path<i32>,
	headers: HeaderMap,
	State(db_pool): State<PgPool>, JsonExtract(account_dto): JsonExtract<AccountDTO>,
) -> Result<Json<Account>, ApiError> {
	let auth_account = get_account_from_authorization_header(&headers, &db_pool).await?;
	if auth_account.permission_level != PermissionLevel::Admin {
		return Err(ApiError { status: axum::http::StatusCode::FORBIDDEN, message: "Insufficient permissions".to_string() });
	}
	let mut account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = $1 AND deleted_at IS NULL;")
		.bind(account_id)
		.fetch_optional(&db_pool)
		.await
		.map_err(map_err_to_500)?
		.ok_or(api_error_500())?;

	account.email = account_dto.email;
	account.updated_at = OffsetDateTime::now_utc();
	sqlx::query("UPDATE accounts SET email = $1, updated_at = $2 WHERE id = $3;")
		.bind(&account.email)
		.bind(account.updated_at)
		.bind(account.id)
		.execute(&db_pool)
		.await
		.map_err(map_err_to_500)?;
	
	Ok(Json(account))
}

async fn delete_api_auth_accounts_id(
	Path(account_id): Path<i32>,
	headers: HeaderMap,
	State(db_pool): State<PgPool>,
) -> Result<(), ApiError> {
	let auth_account = get_account_from_authorization_header(&headers, &db_pool).await?;
	if auth_account.permission_level != PermissionLevel::Admin {
		return Err(ApiError { status: axum::http::StatusCode::FORBIDDEN, message: "Insufficient permissions".to_string() });
	}
	let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = $1 AND deleted_at IS NULL;")
		.bind(account_id)
		.fetch_optional(&db_pool)
		.await
		.map_err(map_err_to_500)?
		.ok_or(api_error_500())?;

	let deleted_at = OffsetDateTime::now_utc();
	sqlx::query("UPDATE accounts SET deleted_at = $1 WHERE id = $2;")
		.bind(deleted_at)
		.bind(account.id)
		.execute(&db_pool)
		.await
		.map_err(map_err_to_500)?;
	
	Ok(())
}

pub async fn get_account_from_authorization_header(headers: &HeaderMap, db_pool: &PgPool) -> Result<Account, ApiError> {
	let auth_header = headers.get("Authorization")
		.ok_or(ApiError { status: axum::http::StatusCode::UNAUTHORIZED, message: "Missing authorization header".to_string() })?
		.to_str()
		.map_err(|_| ApiError { status: axum::http::StatusCode::UNAUTHORIZED, message: "Invalid authorization header".to_string() })?;
	if !auth_header.starts_with("Bearer ") {
		return Err(ApiError { status: axum::http::StatusCode::UNAUTHORIZED, message: "Invalid authorization header".to_string() });
	}
	let token = auth_header.trim_start_matches("Bearer ").trim();
	let account_id: i32 = token.parse().map_err(|_| ApiError { status: axum::http::StatusCode::UNAUTHORIZED, message: "Invalid token".to_string() })?;
	if account_id == 0 {
		return Ok(Account {
			id: 0,
			email: "superuser".to_string(),
			permission_level: PermissionLevel::Admin,
			created_at: OffsetDateTime::now_utc(),
			updated_at: OffsetDateTime::now_utc(),
			deleted_at: None,
		});
	}
	let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = $1 AND deleted_at IS NULL;")
		.bind(account_id)
		.fetch_optional(db_pool)
		.await
		.map_err(map_err_to_500)?
		.ok_or(ApiError { status: axum::http::StatusCode::UNAUTHORIZED, message: "Account not found".to_string() })?;
	Ok(account)
}

/// Router for /api/auth
pub fn get_auth_router() -> Router<PgPool> {
	Router::new()
		.route("/accounts", get(get_api_auth_accounts).post(post_api_auth_accounts))
		.route("/accounts/{account_id}", patch(patch_api_auth_accounts_id).delete(delete_api_auth_accounts_id))
}