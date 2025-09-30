use crate::app::result::AppResult;

mod app;
mod server;
mod routers;
mod controllers;
mod dto;
mod config;
mod bootstrap;
mod utils;
mod middleware;

#[tokio::main]
async fn main() -> AppResult<()> {
    // เริ่มระบบที่จำเป็น
    bootstrap::run();

    server::run().await?;
    Ok(())
}
