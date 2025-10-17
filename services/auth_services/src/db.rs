use sqlx::{MySql, Pool};
use dotenvy::dotenv;
use std::env;

pub async fn connect_db() -> Pool<MySql> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    Pool::<MySql>::connect(&url).await.expect("DB connection failed")
}
