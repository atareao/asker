use serde::{Serialize, Deserialize};

#[derive(sqlx::Type)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Role {
    User,
    Admin,
}

