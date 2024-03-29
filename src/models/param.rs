use serde::{Serialize, Deserialize};
use sqlx::{sqlite::{SqlitePool, SqliteRow}, query, Row};
use chrono::{DateTime, Utc};
use tracing::{info, debug};
use std::collections::HashMap;

// Here my things
use super::Error;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Param{
    id: i64,
    key: String,
    value: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}


impl Param{
    #[allow(dead_code)]
    pub fn get_id(&self) -> i64{
        self.id
    }

    pub fn get_key(&self) -> &str{
        &self.key
    }

    pub async fn get_url(pool: &SqlitePool) -> String{
        Self::get(pool, "url")
            .await
            .unwrap()
    }

    pub async fn get_port(pool: &SqlitePool) -> u16{
        Self::get(pool, "port")
            .await
            .unwrap()
            .parse::<u16>()
            .unwrap()
    }

    pub async fn get_secret(pool: &SqlitePool) -> String{
        Self::get(pool, "jwt_secret")
            .await
            .unwrap()
    }

    pub async fn get_sleep_time(pool: &SqlitePool) -> u64{
        Self::get(pool, "sleep_time")
            .await
            .unwrap()
            .parse::<u64>()
            .unwrap()
    }

    pub async fn get_older_than(pool: &SqlitePool) -> i32{
        Self::get(pool, "older_than")
            .await
            .unwrap()
            .parse::<i32>()
            .unwrap()
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }

    pub fn get_created_at(&self) -> &DateTime<Utc>{
        &self.created_at
    }

    pub fn get_updated_at(&self) -> &DateTime<Utc>{
        &self.updated_at
    }

    fn from_row(row: SqliteRow) -> Self{
        Self{
            id: row.get("id"),
            key: row.get("key"),
            value: row.get("value"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub async fn get(pool: &SqlitePool, key: &str) -> Result<String, Error>{
        debug!("get {key}");
        let sql = "SELECT value FROM config WHERE key = $1";
        query(sql)
            .bind(key)
            .map(|row: SqliteRow| -> String {row.get(0)})
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_all(pool: &SqlitePool) -> Result<HashMap<String, String>, Error>{
        info!("get_all");
        let sql = "SELECT * FROM config";
        let params = query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await?;
        let mut kv = HashMap::new();
        for param in params{
            debug!("{:?}", param);
            kv.insert(param.key, param.value);
        }
        Ok(kv)
    }


    pub async fn exists(pool: &SqlitePool, key: &str) -> Result<bool, Error>{
        debug!("exists {key}");
        let sql = "SELECT count(key) FROM config WHERE key = $1";
        Ok(query(sql)
            .bind(key)
            .map(|row: SqliteRow| -> i64 {row.get(0)})
            .fetch_one(pool)
            .await? > 0)
    }

    pub async fn set(pool: &SqlitePool, key: &str, value: &str) -> Result<Param, Error>{
        debug!("set {key}={value}");
        let current_ts = Utc::now();
        let sql = "INSERT INTO config(key, value, updated_at) \
            VALUES($1, $2, $3)
            ON CONFLICT(key) DO UPDATE SET
            value=excluded.value,
            updated_at=excluded.updated_at
            RETURNING *";
        query(sql)
            .bind(key)
            .bind(value)
            .bind(current_ts)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }
}


