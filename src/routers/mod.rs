use std::sync::Arc;

use axum::{Router, routing::{get, post, put, delete}};

use crate::{app::state::AppState, controllers::{orders::core::create_order, products::core::{create_product, delete_product, get_product, list_products, update_product}, users::core::{create_user, list_users}}};

pub fn api(state: Arc<AppState>) -> Router {
    Router::<Arc<AppState>>::new()
        .route("/api/users", get(list_users))
        .route("/api/users", post(create_user))

        .route("/api/products", get(list_products))
        .route("/api/products", post(create_product))
        .route("/api/products/{id}", get(get_product))
        .route("/api/products/{id}", put(update_product))
        .route("/api/products/{id}", delete(delete_product))

        .route("/api/orders", post(create_order))
        
        .with_state(state)
}

