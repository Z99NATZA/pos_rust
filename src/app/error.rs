#![allow(dead_code)]

use std::env;
use axum::{Json, http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::error;
use serde_json::json;

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
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message): (StatusCode, &str, String) = match &self {
            AppError::InternalServerError(e) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error", e.to_string()),
            
            AppError::IoError(_)
            | AppError::DotEnvError(_)
            | AppError::EnvVarError(_)
            | AppError::SqlxError(_)
                => (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error", "Internal server error".into())
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