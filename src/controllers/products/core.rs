use std::sync::Arc;

use axum::{body::Bytes, extract::{Multipart, State}};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{app::{error::AppError, result::AppResult, state::AppState}, utils::{file::{ensure_valid_ext, validate_image_ext}, numeric::string_to_decimal_2}};

pub async fn create_product(
    State(_state): State<Arc<AppState>>,
    mut mp: Multipart
) -> AppResult<()> {
    let mut name: Option<String> = None;
    let mut code: Option<String> = None;
    let mut price: Option<Decimal> = None;
    let mut description = String::new();
    let mut is_active = true;
    let mut image_name = String::new();
    let mut data = Bytes::new();

    while let Some(field) = mp.next_field().await? {
        match field.name() {
            Some("name") => name = Some(field.text().await?),
            Some("code") => code = Some(field.text().await?),
            Some("price") => {
                let s = field.text().await?;
                price = Some(string_to_decimal_2(s));
            },
            Some("description") => description = field.text().await?,
            Some("is_active") => {
                let s = field.text().await?;
                is_active = s.parse::<bool>().unwrap_or(true);
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

    println!(
        r#"
            Product:
            name = {:?}
            code = {:?}
            price = {:?}
            description = {}
            is_active = {}
            image_name = {}
        "#,
            name,
            code,
            price,
            description,
            is_active,
            image_name
    );

    tokio::fs::write(format!("images/products/{image_name}"), data).await?;

    Ok(())
}