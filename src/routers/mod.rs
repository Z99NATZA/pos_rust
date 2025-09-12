use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{app::state::AppState, controllers::users::core::{create_user, list_users}};

pub fn api(state: Arc<AppState>) -> Router {
    Router::<Arc<AppState>>::new()
        .route("/api/users", get(list_users))
        .route("/api/users", post(create_user))
        .with_state(state)
}

