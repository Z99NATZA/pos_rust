pub mod dir;
pub mod tracing;

pub fn run () {
    if cfg!(debug_assertions) {
        dotenvy::dotenv().ok();
    }

    dir::create_dir();
    tracing::log_debug();
}