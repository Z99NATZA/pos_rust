use std::{env, sync::Arc};

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode, response::{IntoResponse, Response}};
use axum_extra::extract::CookieJar;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use cookie::{Cookie, SameSite, time::Duration as CookieDuration};

use crate::{app::{error::AppError, result::AppResult, state::AppState}, dto::{auth::{Claims, LoginRequest}, base::BaseApiResponse}};

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>
) -> AppResult<Response> {
    let notfound = "อีเมล หรือ รหัสผ่าน ไม่ถูกต้อง";

    let row = sqlx::query!(
        r#"
            SELECT id, username, email, password_hash, role, is_active
            FROM users
            WHERE email = $1
        "#,
        payload.email
    )
    .fetch_optional(&state.db)
    .await?;

    let Some(row) = row else {
        return Err(AppError::UnauthorizedCustom(notfound.into()));
    };

    let parsed_hash = PasswordHash::new(&row.password_hash)?;

    let ok = Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_ok();

    if !ok {
        return Err(AppError::UnauthorizedCustom(notfound.into()));
    }

    if !row.is_active {
        return Err(AppError::UnauthorizedCustom("ไม่มีสิทธิ์ใช้งาน".into()));
    }

    let min = env::var("ACCESS_TOKEN_TTL_MIN")
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(15);

    let now = Utc::now();
    let exp = now + Duration::minutes(min);

    let claims = Claims {
        sub: row.id,
        email: row.email,
        username: row.username,
        role: row.role,
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let jwt_secret = env::var("JWT_SECRET")?;

    let token = encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(jwt_secret.as_ref())
    )?;

    let cookie_domain = env::var("COOKIE_DOMAIN").unwrap_or_default();
    let mut cookie_builder = Cookie::build(("access_token", token.clone()))
        .path("/")                       // ส่งได้ทั้งไซต์
        .http_only(true)                 // JS อ่านไม่ได้
        .same_site(SameSite::None)       // รองรับ cross-site
        .secure(true)                    // ใช้ HTTPS เท่านั้น
        .max_age(CookieDuration::seconds((exp - now).num_seconds()));

    if !cookie_domain.is_empty() {
        cookie_builder = cookie_builder.domain(cookie_domain);
    }

    let auth_cookie = cookie_builder.build();

    let jar = jar.add(auth_cookie);
    
    let res = BaseApiResponse {
        success: true,
        message: Some("login success".into()),
    };

    Ok((jar, (StatusCode::OK, Json(res))).into_response())
}


