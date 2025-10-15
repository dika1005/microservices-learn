// services/user_services/src/main.rs

// Deklarasikan modul-modul agar bisa diakses oleh main.rs
mod handlers;
mod db;
mod seeder;

// ... impor lainnya
use sqlx::mysql::MySqlPool;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").ok();
    
    // Inisialisasi pool koneksi
    let pool: Option<MySqlPool> = if let Some(db_url) = database_url {
        println!("DATABASE_URL found, attempting to connect to database...");
        let pool_result = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await;
        
        match pool_result {
            Ok(pool) => {
                if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
                    eprintln!("Failed to run database migrations: {}", e);
                    std::process::exit(1);
                }
                
                // Opsional: Jalankan seeder
                // if let Err(e) = seeder::run_seeder(&pool).await {
                //     eprintln!("Failed to run seeder: {}", e);
                //     std::process::exit(1);
                // }
                
                Some(pool)
            },
            Err(e) => {
                eprintln!("Failed to connect to MySQL database: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("No DATABASE_URL found. Starting in no-db mode. Create a .env or set DATABASE_URL to enable DB features.");
        None
    };

    // Definisikan rute dan state terpisah
    let app: Router = if let Some(p) = pool {
        // Mode dengan database
        Router::new()
            .route("/users", get(handlers::list_users))
            .route("/users", post(handlers::create_new_user))
            .with_state(p)
    } else {
        // Mode tanpa database
        Router::new()
            .route("/users", get(|| async { "API is running in no-db mode." }))
            .route("/users", post(|| async { "API is running in no-db mode." }))
    };

    // Jalankan server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}