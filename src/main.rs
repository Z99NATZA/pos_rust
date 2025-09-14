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

    server::run().await?;
    Ok(())
}
