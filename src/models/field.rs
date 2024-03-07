use serde::{
    Serialize,
    Deserialize
};
use sqlx::{
    sqlite::{
        SqlitePool,
        SqliteRow
    },
    query,
    Row
};
use chrono::{
    DateTime,
    Utc
};
use tracing::{
    info,
    debug
};

// my own uses
use super::{
    Error,
    default_datetime
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    id: i64,
    form_id: i64,
    name: String,
    datatype: String,
    label: String,
    placeholder: String,
    required: bool,
    unique: bool,
    #[serde(default = "default_datetime")]
    created_at: DateTime<Utc>,
    #[serde(default = "default_datetime")]
    updated_at: DateTime<Utc>,
}

impl Field{
    pub fn get_id(&self) -> i64{
        self.id
    }

    fn from_row(row: SqliteRow) -> Self{
        info!("from_row");
        Self{
            id: row.get("id"),
            form_id: row.get("form_id"),
            name: row.get("key"),
            datatype: row.get("value"),
            label: row.get("created_at"),
            placeholder: row.get("updated_at"),
            required: row.get("required"),
            unique: row.get("unique"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }

    pub fn new(form_id: i64, name: String, datatype: String, label: String,
               placeholder: String, required: bool, unique: bool) -> Self{
        let created_at = Utc::now();
        let updated_at = created_at.clone();
        Self{
            id: -1,
            form_id,
            name,
            datatype,
            label,
            placeholder,
            required,
            unique,
            created_at,
            updated_at,
        }
    }

    pub async fn create(pool: &SqlitePool, field: &Self) -> Result<Self, Error>{
        info!("create");
        let sql = "INSERT INTO field.rss (form_id, name, datetype, label,
                   placeholder, required, unique, created_at, updated_at)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *";
        query(sql)
            .bind(&field.form_id)
            .bind(&field.name)
            .bind(&field.datatype)
            .bind(&field.label)
            .bind(&field.placeholder)
            .bind(&field.required)
            .bind(&field.unique)
            .bind(&field.created_at)
            .bind(&field.updated_at)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
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

    pub async fn delete (&mut self, pool: &SqlitePool) -> Result<Self, Error>{
        info!("remove");
        Self::remove(pool, self.id).await
    }

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        info!("read");
        let sql = "SELECT * FROM field.rss WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn read_by_name(pool: &SqlitePool, name: &str) -> Result<Self, Error>{
        info!("read_by_name");
        let sql = "SELECT * FROM field.rss WHERE name = $1";
        query(sql)
            .bind(name)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn read_by_form_id(pool: &SqlitePool, form_id: i64) -> Result<Vec<Self>, Error>{
        info!("read_by_form_id");
        let sql = "SELECT * FROM field.rss WHERE form_id = $1";
        query(sql)
            .bind(form_id)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Self>, Error>{
        info!("read_all");
        let sql = "SELECT * FROM field.rss";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn update(pool: &SqlitePool, field: &Self) -> Result<Self, Error>{
        info!("update");
        let updated_at = Utc::now();
        let sql = "UPDATE field.rss SET datatype = $1, label = $2, placeholder = $3, required = $4, unique = $5, updated_at = $6 WHERE id = $7 RETURNING *";
        query(sql)
            .bind(&field.datatype)
            .bind(&field.label)
            .bind(&field.placeholder)
            .bind(&field.unique)
            .bind(updated_at)
            .bind(&field.id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
        .map_err(|e| e.into())
    }

    pub async fn remove(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        info!("delete");
        let sql = "DELETE field.rss WHERE id = $1 RETURNING *";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
        .map_err(|e| e.into())
    }

    pub fn to_sqlite(&self) -> &str{
        match self.datatype.as_str(){
            "checkbox"       => "BOOLEAN",
            "color"          => "TEXT",
            "date"           => "DATE",
            "datetime-local" => "DATETIME",
            "email"          => "TEXT",
            "month"          => "INTEGER",
            "number"         => "REAL",
            "password"       => "TEXT",
            "radio"          => "BOOLEAN",
            "range"          => "INTEGER",
            "tel"            => "TEXT",
            "text"           => "TEXT",
            "time"           => "TIME",
            "url"            => "TEXT",
            "week"           => "INTEGER",
            _                => "TEXT",
        }
    }
}
