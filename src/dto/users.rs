use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, FromRow, Serialize)]
pub struct ListUsers {
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, message = "username too short"))]
    pub username: String,

    #[validate(email(message = "invalid email"))]
    pub email: String,

    #[validate(length(min = 8, message = "password too short"))]
    pub password: String,
    
    pub role: Option<String>,
    pub is_active: Option<bool>
}

