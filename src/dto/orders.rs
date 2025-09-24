use chrono::{DateTime, Utc};
use sqlx::{Type, prelude::FromRow};
use serde::{Serialize, Deserialize};

use rust_decimal::Decimal;
use uuid::Uuid;
use crate::utils::deserialize::deserialize_string_to_decimal;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[sqlx(type_name = "payment_method", rename_all = "UPPERCASE")]
pub enum PaymentMethod {
    CASH,
    TRANSFER,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrder {
    pub note: String,
    pub payment_method: PaymentMethod,
    pub order_items: Vec<CreateOrderItems>,

    #[serde(deserialize_with = "deserialize_string_to_decimal")]
    pub grand_total: Decimal,

    #[serde(deserialize_with = "deserialize_string_to_decimal")]
    pub paid_amount: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderItems {
    pub product_id: Uuid,
    pub product_name: String,

    #[serde(deserialize_with = "deserialize_string_to_decimal")]
    pub unit_price: Decimal,

    #[serde(deserialize_with = "deserialize_string_to_decimal")]
    pub qty: Decimal,

    #[serde(deserialize_with = "deserialize_string_to_decimal")]
    pub total_amount: Decimal,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ListOrders {
    pub note: String,
    pub payment_method: PaymentMethod,
    pub grand_total: Decimal,
    pub paid_amount: Decimal,
    pub user_id: Uuid,
    pub change_amount: Decimal,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ListOrderItemsByOrderId {
    order_id: Uuid,
    product_id: Uuid,
    product_name: String,
    unit_price: Decimal,
    qty: Decimal,
    total_amount: Decimal,
    created_at: DateTime<Utc>,
}
