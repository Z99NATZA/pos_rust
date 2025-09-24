#![allow(dead_code)]

use std::env;
use axum::{Json, extract::multipart::MultipartError, http::StatusCode, response::IntoResponse};
use sqlx::postgres::PgDatabaseError;
use thiserror::Error;
use tracing::error;
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal server error")]
    InternalServerError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Env error: {0}")]
    DotEnvError(#[from] dotenvy::Error),

    #[error("Env var error: {0}")]
    EnvVarError(#[from] env::VarError),

    #[error("Sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Argon2 error: {0}")]
    Argon2Error(#[from] argon2::password_hash::Error),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),

    #[error("Multipart error: {0}")]
    MultipartError(#[from] MultipartError),

    #[error("Rust decimal error: {0}")]
    RustDecimalError(#[from] rust_decimal::Error),

    #[error("Bad request")]
    BadRequestCustom(String),

    #[error("Serde json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message): (StatusCode, &'static str, String) = match &self {
            AppError::InternalServerError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_server_error",
                e.clone(),
            ),

            AppError::SqlxError(e) => {
                match e {
                    sqlx::Error::RowNotFound => (
                        StatusCode::NOT_FOUND,
                        "not_found",
                        "ไม่พบข้อมูล".to_string(),
                    ),
                    sqlx::Error::Database(db_err) => {
                        if let Some("23505") = db_err.code().as_deref() {
                            if let Some(pg) = db_err.try_downcast_ref::<PgDatabaseError>() {
                                let field: Option<String> = pg
                                    .detail()
                                    .and_then(|d| d.split("Key (").nth(1))
                                    .and_then(|s| s.split(')').next())
                                    .and_then(|cols| cols.split(',').next())
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty());

                                let msg = match field.as_deref() {
                                    Some("name") => "ชื่อซ้ำ".to_string(),
                                    Some("code") => "code ซ้ำ".to_string(),
                                    _ => "เคยมีอยู่แล้ว".to_string(),
                                };

                                (StatusCode::CONFLICT, "unique_violation", msg)
                            } else {
                                (StatusCode::CONFLICT, "unique_violation", "เคยมีอยู่แล้ว".into())
                            }
                        }
                        else {
                            (StatusCode::INTERNAL_SERVER_ERROR, "database_error", "Internal server error".into())
                        }
                    }
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "database_error",
                        "Internal server error".into(),
                    )
                }
            }
            
            AppError::IoError(_)
            | AppError::DotEnvError(_)
            | AppError::EnvVarError(_) 
            | AppError::Argon2Error(_) 
            | AppError::SerdeJsonError(_) 
            => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_server_error",
                "Internal server error".into(),
            ),

            AppError::ValidationError(e) => {
                /*
                 * ตัดชื่อ field จริง ๆ ออก เอาเฉพาะ message
                 * 
                 * #[validate(length(min = 3, message = "username too short"))]
                 * pub username: String,
                 * 
                 * เดิม "username: username too short"
                 * เหลือ "username too short"
                 */

                let message = e
                    .field_errors()
                    .iter()
                    .flat_map(|(_, errs)| errs.iter())
                    .filter_map(|err| err.message.as_ref())
                    .map(|msg| msg.to_string())
                    .collect::<Vec<_>>()
                    .join(", "); // เผื่อหลาย field
                (
                    StatusCode::BAD_REQUEST,
                    "validation_error",
                    message,
                )
            }

            AppError::MultipartError(e) => (
                StatusCode::BAD_REQUEST,
                "invalid_multipart",
                e.to_string(),
            ),
            AppError::RustDecimalError(e) => (
                StatusCode::BAD_REQUEST,
                "invalid_decimal",
                e.to_string(),
            ),
            AppError::BadRequestCustom(e) => (
                StatusCode::BAD_REQUEST,
                "bad_request",
                e.to_string(),
            ),
        };

        if status.is_server_error() {
            error!(error = ?self, "Internal server error");
        }

        let body = Json(json!({
            "error": {
                "code": code,
                "message": message
            }
        }));

        (status, body).into_response()
    }
}