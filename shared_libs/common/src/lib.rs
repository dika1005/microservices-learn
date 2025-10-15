// shared_libs/common/src/lib.rs

// Import trait-trait yang diperlukan dari `log` dan `env_logger`.
// Ini adalah library standar untuk logging di Rust.
use env_logger::{Builder, Env};
use log::info;

// Fungsi untuk menyiapkan logger.
// Kita bisa memanggil ini di setiap microservice di awal `main.rs`.
pub fn setup_logger() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .init();
    info!("Logger has been set up.");
}