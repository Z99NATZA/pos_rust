use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::{IntoResponse, Response}};

use crate::{app::{result::AppResult, state::AppState}, controllers::users::utils::password_hash, dto::{base::BaseApiResponse, users::{CreateUser, ListUsers}}};

pub async fn list_users(
    State(state): State<Arc<AppState>>
) -> AppResult<Json<Vec<ListUsers>>> {
    let query = sqlx::query_as::<_, ListUsers>(r#"
        SELECT
            username,
            email,
            COALESCE(role, 'staff') as role
        FROM users
    "#)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(query))
}

pub async fn create_user (
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>
) -> AppResult<Response> {
    let password_hash = password_hash(payload.password.clone())?;

    let _ = sqlx::query(r#"
            INSERT INTO users(username, password_hash, email, role, is_active)
            values($1, $2, $3, $4, $5)
        "#)
        .bind(&payload.username)
        .bind(password_hash)
        .bind(&payload.email)
        .bind(&payload.role)
        .bind(payload.is_active)
        .execute(&state.db)
        .await?
        ;

    let res = BaseApiResponse {
        success: true,
        message: Some("Created".into())
    };

    println!("{res:?}");

    Ok((StatusCode::CREATED, Json(res)).into_response())
}

