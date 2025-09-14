use rust_decimal::Decimal;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProduct {
    #[validate(length(min = 3, max = 20, message = "code สั้นเกินไป 3-20"))]
    pub code: String,

    #[validate(length(min = 2, max = 50, message = "ชื่อ สั้นเกินไป 2-50"))]
    pub name: String,

    pub image_name: String,

    pub description: Option<String>,
    
    pub price: Option<Decimal>,

    pub is_active: Option<bool>,
} 