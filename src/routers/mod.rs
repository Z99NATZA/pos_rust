use std::sync::Arc;

use axum::{Router, routing::{get, post, put}};

use crate::{app::state::AppState, controllers::{products::core::{create_product, get_product_by_code, list_products, update_product}, users::core::{create_user, list_users}}};

pub fn api(state: Arc<AppState>) -> Router {
    Router::<Arc<AppState>>::new()
        .route("/api/users", get(list_users))
        .route("/api/users", post(create_user))
        .route("/api/products", get(list_products))
        .route("/api/products", post(create_product))
        .route("/api/products/{code}", get(get_product_by_code))
        .route("/api/products/{code}", put(update_product))
        .with_state(state)
}

