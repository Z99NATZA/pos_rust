use std::sync::Arc;

use axum::{Router, routing::{get, post}};

use crate::{app::state::AppState, controllers::{products::core::{create_product, get_product_by_code, list_products}, users::core::{create_user, list_users}}};

pub fn api(state: Arc<AppState>) -> Router {
    Router::<Arc<AppState>>::new()
        .route("/api/users", get(list_users))
        .route("/api/users", post(create_user))
        .route("/api/products", get(list_products))
        .route("/api/products", post(create_product))
        .route("/api/products/{id}", get(get_product_by_code))
        .with_state(state)
}

