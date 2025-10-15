// shared_libs/shared_types/src/lib.rs

// Import trait-trait yang diperlukan dari Serde dan SQLx.
// Serde untuk serialization (ke JSON) dan deserialization (dari JSON).
// SQLx untuk mapping dari hasil query database.
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Struct `User` merepresentasikan data pengguna yang akan diambil dari database.
// `FromRow` memungkinkan SQLx untuk mengisi struct ini secara langsung dari baris tabel.
// `Serialize` memungkinkan kita mengubah struct ini menjadi format seperti JSON untuk respons API.
#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

// Struct `NewUser` merepresentasikan data pengguna baru yang dikirimkan melalui request API.
// `Deserialize` memungkinkan kita mengambil data JSON dari request dan memasukkannya ke struct ini.
#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

// Struct `UserPayload` ini opsional, tapi berguna untuk API yang lebih fleksibel.
// Kita bisa menggunakan ini untuk request yang tidak hanya membuat user baru tapi juga bisa update.
#[derive(Debug, Deserialize)]
pub struct UserPayload {
    pub name: Option<String>,
    pub email: Option<String>,
}