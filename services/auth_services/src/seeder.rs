// src/seeder.rs
use sqlx::{MySqlPool, query};
use bcrypt::{hash, DEFAULT_COST};

pub async fn seed_data(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    // Seed roles
    let roles = vec!["admin", "user"];
    for role in &roles {
        query("INSERT IGNORE INTO roles (name) VALUES (?)")
            .bind(role)
            .execute(pool)
            .await?;
    }

    // Cek apakah admin sudah ada
    let admin_exist: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE email = 'admin@mail.com'")
        .fetch_optional(pool)
        .await?;

    if admin_exist.is_none() {
        let hashed_password = hash("admin123", DEFAULT_COST).unwrap();
        query("INSERT INTO users (username, email, password, role_id) VALUES (?, ?, ?, ?)")
            .bind("admin")
            .bind("admin@mail.com")
            .bind(hashed_password)
            .bind(1) // role_id admin
            .execute(pool)
            .await?;
        println!("✅ Default admin created (email: admin@mail.com, pass: admin123)");
    } else {
        println!("ℹ️ Admin already exists, skipping...");
    }

    Ok(())
}
