use std::env;

use axum::{
    extract::Request,
    http::{HeaderMap, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::{app::{error::AppError, result::AppResult}, dto::auth::Claims};

// Middleware ตรวจสอบ JWT
pub async fn auth(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> AppResult<Response> {
    match get_token(&headers).and_then(|t| verify_token(t).ok()) {
        Some(_claims) => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => Err(AppError::Unauthorized),
    }
}

// ดึง token จาก Header: Authorization: Bearer <token>
fn get_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
}

// ตรวจสอบความถูกต้องของ token
fn verify_token(token: &str) -> AppResult<Claims> {
    let jwt_secret = env::var("JWT_SECRET")?;

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(data.claims)
}

/*
https://docs.rs/axum/latest/axum/middleware/fn.from_fn.html
https://docs.rs/axum/latest/axum/middleware/index.html
 */