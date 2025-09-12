use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{app::{result::AppResult, state::AppState}, dto::users::{CreateUser, ListUsers}};

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
) -> AppResult<()> {
    let _ = sqlx::query(r#"
            INSERT INTO users(username, password, email, role, is_active)
            values($1, $2, $3, $4, $5)
        "#)
        .bind(payload.username)
        .bind(payload.password)
        .bind(payload.role)
        .bind(payload.is_active)
        .execute(&state.db)
        .await?
        ;

    println!("Created");

    Ok(())
}