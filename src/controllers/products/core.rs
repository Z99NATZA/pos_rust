use std::sync::Arc;

use axum::{body::Bytes, extract::{Multipart, State}};
use rust_decimal::Decimal;
use uuid::Uuid;
use validator::Validate;

use crate::{app::{error::AppError, result::AppResult, state::AppState}, dto::products::CreateProduct, utils::{file::{ensure_valid_ext, validate_image_ext}, numeric::string_to_decimal_2}};

pub async fn create_product(
    State(_state): State<Arc<AppState>>,
    mut mp: Multipart
) -> AppResult<()> {
    let mut name = String::new();
    let mut code = String::new();
    let mut price: Decimal = Decimal::ZERO;
    let mut description: Option<String> = None;
    let mut is_active: Option<bool> = None;
    let mut image_name = String::new();
    let mut data = Bytes::new();

    while let Some(field) = mp.next_field().await? {
        match field.name() {
            Some("name") => name = field.text().await?,
            Some("code") => code = field.text().await?,
            Some("price") => {
                let s = field.text().await?;
                price = string_to_decimal_2(s);
            },
            Some("description") => description = Some(field.text().await?),
            Some("is_active") => {
                let s = field.text().await?;
                is_active = Some(s.parse::<bool>().unwrap_or(true));
            },
            Some("image_file") => {
                let filename = field
                    .file_name()
                    .ok_or_else(|| AppError::BadRequestCustom("เพิ่มภาพสินค้า".into()))?;

                let ext = ensure_valid_ext(filename)?;
                let _ = validate_image_ext(ext.as_str())?;

                image_name = format!("product-{}.{}", Uuid::new_v4(), ext);

                data = field.bytes().await?;
            },
            _ => {}
        }
    }

    let dto = CreateProduct {
        name: name,
        code: code,
        price: price,
        description: description,
        image_name: image_name.clone(),
        is_active: is_active
    };

    dto.validate()?;

    println!("{dto:#?}");

    tokio::fs::write(format!("images/products/{image_name}"), data).await?;

    Ok(())
}