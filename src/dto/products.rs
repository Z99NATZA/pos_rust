use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProduct {
    #[validate(length(min = 3, max = 20, message = "code สั้นเกินไป 3-20"))]
    pub code: String,

    #[validate(length(min = 2, max = 50, message = "ชื่อ สั้นเกินไป 2-50"))]
    pub name: String,

    #[validate(length(min = 1, message = "เพิ่มภาพสินค้า"))]
    pub image_name: String,

    pub description: Option<String>,
    
    pub price: Decimal,

    pub is_active: Option<bool>,
} 

#[derive(Debug, Serialize, FromRow)]
pub struct ListProducts {
    code: String,
    name: String,
    description: Option<String>,
    price: Decimal,
    is_active: bool,
    image_name: String,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ListProductFilter {
    pub is_active: Option<bool>
}