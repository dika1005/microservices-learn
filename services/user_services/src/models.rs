// services/user_services/src/models.rs
use serde::{Deserialize, Serialize};

// Contoh model data yang hanya digunakan di user_service.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub bio: String,
}