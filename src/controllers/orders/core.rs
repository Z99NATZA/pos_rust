use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use rust_decimal::Decimal;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;
use axum::response::Response;

use crate::{
    app::{error::AppError, result::AppResult, state::AppState},
    dto::{base::BaseApiResponse, orders::{CreateOrder, ListOrderItemsByOrderId, ListOrders, PaymentMethod}},
};

pub async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateOrder>,
) -> AppResult<Response> {
    let mut tx: Transaction<'_, Postgres> = state.db.begin().await?;

    let user_id: Uuid = Uuid::parse_str("3a4eb578-91cd-4354-bdf0-540bed747f4e")
        .map_err(|_| AppError::InternalServerError("invalid uuid".into()))?;

    let order_id: Uuid = sqlx::query_scalar!(
        r#"
            INSERT INTO orders(note, grand_total, payment_method, paid_amount, user_id)
            VALUES($1, $2, $3, $4, $5)
            RETURNING id
        "#,
        payload.note,
        payload.grand_total,
        payload.payment_method as PaymentMethod,
        payload.paid_amount,
        user_id
    )
    .fetch_one(&mut *tx)
    .await?;

    let pids: Vec<Uuid> = payload.order_items.iter().map(|x| x.product_id).collect();
    let names: Vec<String> = payload.order_items.iter().map(|x| x.product_name.clone()).collect();
    let unitp: Vec<Decimal> = payload.order_items.iter().map(|x| x.unit_price).collect();
    let qtys: Vec<Decimal> = payload.order_items.iter().map(|x| x.qty).collect();
    let tots: Vec<Decimal> = payload.order_items.iter().map(|x| x.total_amount).collect();

    sqlx::query!(
        r#"
            INSERT INTO order_items(order_id, product_id, product_name, unit_price, qty, total_amount)
            SELECT $1, pid, pname, up, q, ta
            FROM UNNEST($2::uuid[], $3::text[], $4::numeric[], $5::numeric[], $6::numeric[]) 
            AS t(pid, pname, up, q, ta)
        "#,
        order_id,
        &pids,
        &names,
        &unitp,
        &qtys,
        &tots
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let res = BaseApiResponse {
        success: true,
        message: Some(format!("Created. Order id {order_id}").into())
    };

    Ok((StatusCode::CREATED, Json(res)).into_response())
}

pub async fn list_orders(
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<Vec<ListOrders>>> {
    let rows = sqlx::query_as::<_, ListOrders>(
        r#"
            SELECT
                note,
                payment_method,
                grand_total,
                paid_amount,
                user_id,
                change_amount
            FROM orders
        "#
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(rows))
}

pub async fn list_order_items_by_order_id(
    State(state): State<Arc<AppState>>,
    Path(order_id): Path<Uuid>
) -> AppResult<Json<Vec<ListOrderItemsByOrderId>>> {
    let rows = sqlx::query_as::<_, ListOrderItemsByOrderId>(
        r#"
            SELECT
                order_id,
                product_id,
                product_name,
                unit_price,
                qty,
                total_amount,
                created_at
            FROM order_items
            WHERE order_id = $1
        "#
    )
    .bind(order_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(rows))
}