use std::{env, sync::Arc};

use tokio::signal;

use crate::{app::{result::AppResult, state::AppState}, config::db::connect, routers};

pub async fn run() -> AppResult<()> {
    let db_url = env::var("DATABASE_URL")?;
    let db = connect(&db_url).await?;

    let state = Arc::new(AppState {
        db
    });

    let app = routers::api(state);

    let host = env::var("HOST")?;
    let port = env::var("PORT")?;
    let addr  = format!("{host}:{port}");

    println!("App running on: {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = signal::ctrl_c().await;
            println!("App offline");
        })
        .await?;

    Ok(())
}