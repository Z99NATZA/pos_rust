use std::sync::Arc;

use axum::extract::State;

use crate::app::{result::AppResult, state::AppState};

pub async fn list_users(
    State(state): State<Arc<AppState>>
) -> AppResult<()> {
    let query = sqlx::query(r#"
        SELECT username FROM users
    "#)
    .fetch_optional(&state.db)
    .await?;

    println!("{query:?}");

    Ok(())
}

