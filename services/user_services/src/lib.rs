// services/user_services/src/lib.rs

// Deklarasi dan ekspos modul-modul di dalam service ini.
// Dengan begini, `main.rs` bisa langsung mengimpornya.
pub mod db;
pub mod handlers;
pub mod models;
pub mod seeder;