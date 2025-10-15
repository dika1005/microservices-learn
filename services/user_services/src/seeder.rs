// services/user_services/src/seeder.rs
use sqlx::MySqlPool;
use crate::db::create_user;
use shared_types::NewUser;

pub async fn run_seeder(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    println!("Running user service seeder...");

    let new_user1 = NewUser {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };
    let new_user2 = NewUser {
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
    };

    create_user(pool, new_user1).await?;
    create_user(pool, new_user2).await?;

    println!("Seeder finished successfully!");
    Ok(())
}