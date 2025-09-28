use std::sync::Arc;

use axum::http::StatusCode;
use axum::{Json, extract::State};
use axum::response::{IntoResponse, Response};

use crate::dto::base::BaseApiResponse;
use crate::{app::{result::AppResult, state::AppState}, dto::users::CreateUser, utils::password::password_hash};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>
) -> AppResult<Response> {
    let password_hash = password_hash(payload.password.clone())?;

    let role = match payload.role {
        Some(ref v) if !v.trim().is_empty() => v.to_string(),
        _ => "stuff".to_string(),
    };

    let is_active = match payload.is_active {
        Some(v) => v,
        _ => true,
    };
 
    let id = sqlx::query_scalar!(
        r#"
            INSERT INTO users(username, email, password_hash, role, is_active)
            VALUES($1, $2, $3, $4, $5)
            RETURNING id
        "#,
        payload.username,
        payload.email,
        password_hash,
        role,
        is_active
    )
    .fetch_one(&state.db)
    .await?;

    let res = BaseApiResponse {
        success: true,
        message: Some(format!("Created. id {id}").into())
    };

    Ok((StatusCode::CREATED, Json(res)).into_response())
}