use sqlx::Type;
use serde::{Serialize, Deserialize};

use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[sqlx(type_name = "payment_method", rename_all = "UPPERCASE")]
pub enum PaymentMethod {
    CASH,
    TRANSFER,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrder {
    pub note: Option<String>,
    pub grand_total: Decimal,
    pub payment_method: PaymentMethod,
    pub paid_amount: Decimal,
    pub users_code: String,
    pub order_items: Vec<CreateOrderItems>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderItems {
    pub product_id: Uuid,
    pub product_name: String,
    pub unit_price: Decimal,
    pub qty: Decimal,
    pub total_amount: Decimal,
}
