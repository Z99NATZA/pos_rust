use std::{env, sync::Arc};

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode, response::{IntoResponse, Response}};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{app::{error::AppError, result::AppResult, state::AppState}, dto::auth::{Claims, LoginRequest, LoginResponse}};

pub async fn login(
    State(state): State<Arc<AppState>>,
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
    
    let res = LoginResponse { 
        access_token: token.clone(), 
        token_type: "Bearer".into(), 
        expires_in: (exp - now).num_seconds() 
    };

    Ok((StatusCode::OK, Json(res)).into_response())
}


