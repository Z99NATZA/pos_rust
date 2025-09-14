use crate::app::result::AppResult;

mod app;
mod server;
mod routers;
mod controllers;
mod dto;
mod config;
mod bootstrap;

#[tokio::main]
async fn main() -> AppResult<()> {
    // เริ่มระบบ
    bootstrap::run();

    // Dev log debug
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    server::run().await?;
    Ok(())
}
