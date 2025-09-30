use std::sync::Arc;

use axum::{Router, middleware, routing::{delete, get, post, put}};
use crate::middleware as my_middleware;
use crate::{app::state::AppState, controllers::{auth::{login::login, register::register}, orders::core::{create_order, list_order_items_by_order_id, list_orders}, products::core::{create_product, delete_product, get_product, list_products, update_product}, users::core::{create_user, list_users}}};

pub fn api(state: Arc<AppState>) -> Router {
    Router::<Arc<AppState>>::new()
        .route("/api/users", get(list_users))
        .route("/api/users", post(create_user))

        .route("/api/products", get(list_products))
        .route("/api/products", post(create_product))
        .route("/api/products/{id}", get(get_product))
        .route("/api/products/{id}", put(update_product))
        .route("/api/products/{id}", delete(delete_product))

        .route("/api/orders", get(list_orders))
        .route("/api/orders", post(create_order))
        .route("/api/orders/order-items/{order_id}", get(list_order_items_by_order_id))
        .route_layer(middleware::from_fn(my_middleware::auth::auth))
        
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))

        .with_state(state)
}

