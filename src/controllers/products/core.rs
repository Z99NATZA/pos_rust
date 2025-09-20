use std::sync::Arc;

use axum::{Json, body::Bytes, extract::{Multipart, Path, Query, State}, http::StatusCode, response::Response};
use rust_decimal::Decimal;
use uuid::Uuid;
use validator::Validate;
use axum::response::IntoResponse;

use crate::{app::{error::AppError, result::AppResult, state::AppState}, dto::{base::BaseApiResponse, products::{CreateProduct, GetProduct, ListProductFilter, ListProducts, UpdateProduct}}, utils::{file::{ensure_valid_ext, validate_image_ext}, numeric::string_to_decimal_2}};

pub async fn list_products(
    State(state): State<Arc<AppState>>,
    Query(filter): Query<ListProductFilter>
) -> AppResult<Json<Vec<ListProducts>>> {
    let query = sqlx::query_as::<_, ListProducts>(r#"
            SELECT
                name, code, description, price,
                is_active, image_name, created_at
            FROM products
            WHERE ($1::boolean IS NULL OR is_active = $1)
            ORDER BY created_at DESC
        "#)
        .bind(filter.is_active)
        .fetch_all(&state.db)
        .await?;

    Ok(Json(query))
}

pub async fn create_product(
    State(state): State<Arc<AppState>>,
    mut mp: Multipart
) -> AppResult<Response> {
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
                validate_image_ext(ext.as_str())?;

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

    sqlx::query(r#"
            INSERT INTO products(name, code, price, description, is_active, image_name)
            values($1, $2, $3, $4, $5, $6)
        "#)
        .bind(dto.name)
        .bind(dto.code)
        .bind(dto.price)
        .bind(dto.description)
        .bind(dto.is_active.unwrap_or(true))
        .bind(dto.image_name)
        .execute(&state.db)
        .await?
        ;

    tokio::fs::write(format!("images/products/{image_name}"), data).await?;

    let res = BaseApiResponse {
        success: true,
        message: Some("Created".into())
    };

    Ok((StatusCode::CREATED, Json(res)).into_response())
}

pub async fn get_product_by_code(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>
) -> AppResult<Json<GetProduct>> {
    let query = sqlx::query_as::<_, GetProduct>(r#"
            SELECT
                code, name, description, price, is_active, image_name
            FROM products
            WHERE code = $1
        "#)
        .bind(code)
        .fetch_one(&state.db)
        .await?
        ;

    Ok(Json(query))
}

pub async fn update_product(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
    mut mp: Multipart
) -> AppResult<StatusCode> {
    let mut name = String::new();
    let mut description: Option<String> = None;
    let mut price = Decimal::ZERO;
    let mut is_active = true;
    let mut image_name_old = String::new();
    let mut image_name: Option<String> = None;
    let mut data = Bytes::new();

    while let Some(field) = mp.next_field().await? {
        match field.name() {
            Some("name") => name = field.text().await?,
            Some("description") => description = Some(field.text().await?),
            Some("price") => {
                let s = field.text().await?;
                price = string_to_decimal_2(s);
            },
            Some("is_active") => {
                let s = field.text().await?;
                is_active = s.parse::<bool>().unwrap_or(true);
            },
            Some("image_name_old") => image_name_old = field.text().await?,
            Some("image_file") => {
                if let Some(filename) = field.file_name() {
                    let ext = ensure_valid_ext(filename)?;
                    validate_image_ext(&ext)?;

                    image_name = Some(format!("product-{}.{}", Uuid::new_v4(), ext));

                    data = field.bytes().await?;
                }
            }
            _ => {}
        }
    }

    let final_image_name = image_name.clone().unwrap_or(image_name_old.clone());

    let dto = UpdateProduct {
        name: name,
        description: description,
        price: price,
        is_active: is_active,
        image_name: final_image_name
    };

    dto.validate()?;

    sqlx::query(r#"
            UPDATE products SET
                name = $2,
                description = $3,
                price = $4,
                is_active = $5,
                image_name = $6
            WHERE code = $1
        "#)
        .bind(code)
        .bind(dto.name)
        .bind(dto.description)
        .bind(dto.price)
        .bind(dto.is_active)
        .bind(dto.image_name)
        .execute(&state.db)
        .await?
        ;

    if let Some(new_name) = image_name {
        tokio::fs::write(format!("images/products/{new_name}"), data).await?;
        
        if !image_name_old.is_empty() {
            println!("{image_name_old}");
            
            let old_path = format!("images/products/{image_name_old}");
            let _ = tokio::fs::remove_file(&old_path).await;
        }
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_product(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>
) -> AppResult<StatusCode> {
    sqlx::query("DELETE FROM products WHERE code = $1")
        .bind(code)
        .execute(&state.db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}