use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct ListUsers {
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
    pub is_active: Option<bool>
}

