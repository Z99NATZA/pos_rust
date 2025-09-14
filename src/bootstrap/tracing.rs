pub fn log_debug() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}