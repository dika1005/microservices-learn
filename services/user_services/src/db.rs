// services/user_services/src/db.rs
use sqlx::MySqlPool;
use shared_types::{User, NewUser}; 
// ...
// Fungsi untuk membuat user baru.
pub async fn create_user(pool: &MySqlPool, new_user: NewUser) -> Result<User, sqlx::Error> {
    // Jalankan query INSERT tanpa RETURNING
    let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(&new_user.name)
        .bind(&new_user.email)
        .execute(pool)
        .await?;

    // Dapatkan ID yang baru dibuat dari hasil eksekusi
    let last_insert_id = result.last_insert_id();

    // Jalankan query SELECT untuk mengambil data lengkap berdasarkan ID
    let user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(last_insert_id)
        .fetch_one(pool)
        .await?;
    
    Ok(user)
}
pub async fn get_all_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(pool)
        .await?;
    
    Ok(users)
}