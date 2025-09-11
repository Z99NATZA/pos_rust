use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{app::{result::AppResult, state::AppState}, dto::users::ListUsers};

pub async fn list_users(
    State(state): State<Arc<AppState>>
) -> AppResult<Json<Vec<ListUsers>>> {
    let query = sqlx::query_as::<_, ListUsers>(r#"
        SELECT
            username,
            email,
            COALESCE(role, 'user') as role
        FROM users
    "#)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(query))
}

