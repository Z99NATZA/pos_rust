use crate::app::result::AppResult;

mod app;
mod server;
mod routers;
mod controllers;
mod dto;

#[tokio::main]
async fn main() -> AppResult<()> {
    server::run().await?;
    Ok(())
}
