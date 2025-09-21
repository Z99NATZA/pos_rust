use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{app::state::AppState, dto::orders::CreateOrder};

pub async fn create_order (
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<CreateOrder>,
) {
    dbg!(&payload);
}