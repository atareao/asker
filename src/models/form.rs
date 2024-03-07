use serde::{Serialize, Deserialize};
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
    Field,
    default_datetime
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Form{
    pub id: i64,
    pub name: String,
    pub title: String,
    pub instructions: String,
    #[serde(default = "default_datetime")]
    created_at: DateTime<Utc>,
    #[serde(default = "default_datetime")]
    updated_at: DateTime<Utc>,
}


impl Form {
    fn from_row(row: SqliteRow) -> Self{
        info!("from_row");
        Self{
            id: row.get("id"),
            name: row.get("name"),
            title: row.get("title"),
            instructions: row.get("instructions"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
    pub fn new(name: String, title: String, instructions: String) -> Self{
        let created_at = Utc::now();
        let updated_at = created_at.clone();
        Self{
            id: -1,
            name,
            title,
            instructions,
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

    pub async fn get_fields(self, pool: &SqlitePool) -> Result<Vec<Field>, Error>{
        info!("get_fields");
        Field::read_by_form_id(pool, self.id).await
    }

    pub async fn create(pool: &SqlitePool, form: &Self) -> Result<Self, Error>{
        info!("create");
        let sql = "INSERT INTO forms (name, title, instructions,
                   created_at, updated_at) VALUES ($1, $2, $3, $4, $5)
                   RETURNING *";
        query(sql)
            .bind(&form.name)
            .bind(&form.title)
            .bind(&form.instructions)
            .bind(&form.created_at)
            .bind(&form.updated_at)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn read(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        info!("read");
        let sql = "SELECT * FROM forms WHERE id = $1";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn read_by_name(pool: &SqlitePool, name: &str) -> Result<Self, Error>{
        info!("read_by_name");
        let sql = "SELECT * FROM forms WHERE name = $1";
        query(sql)
            .bind(name)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn read_all(pool: &SqlitePool) -> Result<Vec<Self>, Error>{
        info!("read_all");
        let sql = "SELECT * FROM forms";
        query(sql)
            .map(Self::from_row)
            .fetch_all(pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn update(pool: &SqlitePool, form: &Self) -> Result<Self, Error>{
        info!("update");
        let updated_at = Utc::now();
        let sql = "UPDATE forms SET title = $1, instructions = $2,
                   updated_at = $3 WHERE id = $4 RETURNING *";
        query(sql)
            .bind(&form.title)
            .bind(&form.instructions)
            .bind(updated_at)
            .bind(form.id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
        .map_err(|e| e.into())
    }

    pub async fn remove(pool: &SqlitePool, id: i64) -> Result<Self, Error>{
        info!("remove");
        let sql = "DELETE forms WHERE id = $1 RETURNING *";
        query(sql)
            .bind(id)
            .map(Self::from_row)
            .fetch_one(pool)
            .await
        .map_err(|e| e.into())
    }

    fn to_sqlite(datatype: &str) -> &str{
        match datatype{
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

    pub fn drop(&self, name: &str) -> String{
        format!("DROP TABLE IF EXISTS {};", name)
    }
}

