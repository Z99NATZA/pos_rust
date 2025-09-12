use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::app::result::AppResult;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}
