use axum::{routing::post, Router};
use tower_cookies::CookieManagerLayer;
use dotenvy::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

use auth_services::{db::connect_db, handlers::*, seeder::seed_data};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    seed_data(&pool).await?;

    let app = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .with_state(pool.clone())
        .layer(CookieManagerLayer::new());

    println!("🚀 Auth service running on http://127.0.0.1:3000");
    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000").await?,
        app,
    )
    .await?;

    Ok(())
}
