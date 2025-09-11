use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct ListUsers {
    pub username: String,
    pub email: String,
    pub role: String,
}