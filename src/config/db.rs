use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::app::result::AppResult;

pub async fn connect(db_url: &str) -> AppResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await?;

    Ok(pool)
}