use std::env;

use axum::{
    extract::Request,
    http::{HeaderMap, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use cookie::Cookie;
use crate::{
    app::{error::AppError, result::AppResult},
    dto::auth::Claims,
};

// Middleware ตรวจสอบ JWT (Bearer หรือ Cookie)
pub async fn auth(
    mut request: Request,
    next: Next,
) -> AppResult<Response> {
    let headers = request.headers();

    // 1) หาจาก Authorization: Bearer ...
    let token = get_token_from_auth(headers)
        // 2) ถ้าไม่มี ลองหาใน Cookie: access_token=...
        .or_else(|| get_token_from_cookie(headers));

    let Some(token) = token else {
        return Err(AppError::Unauthorized);
    };

    let claims = verify_token(&token)?;

    // แนบ claims เข้าไปให้ downstream ใช้งาน
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

// ดึง token จาก Header: Authorization: Bearer <token>
fn get_token_from_auth(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(|s| s.to_string())
}

// ดึง token จาก Cookie header: access_token=<jwt>
fn get_token_from_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    // header Cookie อาจมีหลายค่า; ใช้ Cookie::split_parse ช่วยแยก
    for c in Cookie::split_parse(cookie_header) {
        if let Ok(c) = c {
            if c.name() == "access_token" {
                return Some(c.value().to_string());
            }
        }
    }
    None
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
https://docs.rs/http/latest/http/request/struct.Request.html#method.extensions_mut
https://docs.rs/cookie/latest/cookie/
https://docs.rs/cookie/latest/cookie/struct.Cookie.html#method.split_parse
https://docs.rs/crate/jsonwebtoken/latest
https://docs.rs/crate/jsonwebtoken/latest#method.from_secret
https://docs.rs/http/latest/http/header/index.html
 */