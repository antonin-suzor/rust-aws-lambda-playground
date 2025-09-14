use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, patch},
    Router,
};
use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row, Error as SqlxError, Type as SqlxType};

use crate::utils::{api_error_500, map_err_to_500, ApiError, JsonExtract};

#[derive(Debug, Deserialize, Serialize, SqlxType)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "permission_level_enum", rename_all = "lowercase")]
enum PermissionLevel {
	User,
	Admin,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
struct Account {
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
	State(db_pool): State<PgPool>, JsonExtract(account_dto): JsonExtract<AccountDTO>,
) -> Result<Json<Account>, ApiError> {
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
	State(db_pool): State<PgPool>,
) -> Result<(), ApiError> {
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

/// Router for /api/auth
pub fn get_auth_router() -> Router<PgPool> {
	Router::new()
		.route("/accounts", get(get_api_auth_accounts).post(post_api_auth_accounts))
		.route("/accounts/{account_id}", patch(patch_api_auth_accounts_id).delete(delete_api_auth_accounts_id))
}