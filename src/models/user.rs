use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::{
    sqlite::{
        SqlitePool,
        SqliteRow
    },
    query,
    Row,
};
use tracing::info;
use super::{
    Error,
    Role,
    default_datetime, Param,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User{
    id: i64,
    pub name: String,
    pub hashed_password: String,
    pub role: Role,
    pub active: bool,
    #[serde(default = "default_datetime")]
    created_at: DateTime<Utc>,
    #[serde(default = "default_datetime")]
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct UserSchema {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: i64,
    pub name: String,
    pub role: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn async wrap(pool: &SqlitePool, word: &str) -> String{
    let salt = Param::get("salt").await?;
    let pepper = Param::get("pepper").await?;
    let composition = format!("{}{}{}", salt, word, pepper);
    format!("{:x}", md5::compute(composition))
}


impl User{
    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            name: row.get("name"),
            hashed_password: row.get("hashed_password"),
            role: row.get("role"),
            active: row.get("active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub fn new(pool: &SqlitePool, name: String, password: String, role: Role, active: bool) -> Self{
        let salt = Param::get_key("salt");
        let hashed_password = "";
        let created_at = Utc::now();
        let updated_at = created_at.clone();
        Self{
            id: -1,
            name,
            hashed_password,
            role,
            active,
            created_at,
            updated_at,
        }
    }

    pub async fn save(&mut self, pool: &SqlitePool) -> Result<Self, Error>{
        info!("save");
        if self.id > -1 {
            let saved = Self::update(pool, self).await?;
            self.updated_at = saved.updated_at;
            Ok(saved)
        }else{
            let saved = Self::create(pool, self).await?;
            self.id = saved.id;
            Ok(saved)
        }
    }

    pub async fn delete(&mut self, pool: &SqlitePool) -> Result<Self, Error>{
        info!("remove");
        Self::remove(pool, self.id).await
    }

    pub async fn create(pool: &SqlitePool, user: &Self) -> Result<Self, Error>{
        info!("create");
        let sql = "INSERT INTO users (name, password, role, active, created_at,
                   updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *";
        query(sql)
            .bind(&user.name)
            .bind(&user.password)
            .bind(&user.role)
            .bind(&user.active)
            .bind(&user.created_at)
            .bind(&user.updated_at)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_by_name(pool: &SqlitePool, name: &str) -> Result<User, Error>{
        let sql = "SELECT * FROM users WHERE name = $1";
        query(sql)
            .bind(name)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }
}

