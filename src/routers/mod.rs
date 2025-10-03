use std::sync::Arc;

use crate::middleware as my_middleware;
use crate::{
    app::state::AppState,
    controllers::{
        auth::{login::login, register::register},
        orders::core::{create_order, list_order_items_by_order_id, list_orders},
        products::core::{
            create_product, delete_product, get_product, list_products, update_product,
        },
        users::core::{create_user, list_users},
    },
};
use axum::{
    Router,
    http::{HeaderValue, Method},
    middleware,
    routing::{delete, get, post, put},
};
use tower_http::cors::CorsLayer;

pub fn api(state: Arc<AppState>) -> Router {
    // ตั้งค่า CORS ให้รับ credentials + ระบุ origin ตายตัว (ห้าม '*')
    let frontend_origin =
        std::env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "https://app.example.com".to_string());

    let cors = CorsLayer::new()
        .allow_origin(frontend_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
            axum::http::header::AUTHORIZATION,
        ])
        .allow_credentials(true);

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
        .route(
            "/api/orders/order-items/{order_id}",
            get(list_order_items_by_order_id),
        )

        .route_layer(middleware::from_fn(my_middleware::auth::auth))

        .route("/auth/register", post(register))
        .route("/auth/login", post(login))

        .layer(cors)
        .with_state(state)
}
