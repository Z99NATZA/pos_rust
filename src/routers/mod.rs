use std::sync::Arc;

use axum::{Router, routing::{get}};

use crate::{app::state::AppState, controllers::users::core::list_users};

pub fn api(state: Arc<AppState>) -> Router {
    Router::<Arc<AppState>>::new()
        .route("/api/users", get(list_users))
        .with_state(state)
}

