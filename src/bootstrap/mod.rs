pub mod dir;

pub fn run () {
    if cfg!(debug_assertions) {
        dotenvy::dotenv().ok();
    }

    dir::create_dir();
}