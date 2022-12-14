use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_yaml::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field{
    pub name: String,
    pub datatype: String,
    pub label: String,
    pub placeholder: String,
    pub required: bool,
    pub unique: bool,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Table{
    pub template: String,
    pub title: String,
    pub instructions: String,
    pub fields: Vec<Field>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration{
    pub log_level: String,
    pub db_url: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub tables: HashMap<String, Table>,
}

impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, Error>{
        serde_yaml::from_str(content)
    }

    pub fn get_log_level(&self) -> &str{
        &self.log_level
    }

    pub fn get_db_url(&self) -> &str{
        &self.db_url
    }

    pub fn get_port(&self) -> &str{
        &self.port
    }

    pub fn get_table(&self, table: &str) -> Option<&Table>{
        self.tables.get(&table.to_string())
    }
}

impl Table {
    pub fn create(&self, name: &str) -> String{
        let fields = self.get_fields();
        format!("CREATE TABLE IF NOT EXISTS {} ({});", name, fields)
    }

    pub fn insert(&self, name: &str) -> String{
        let fields: Vec<String> = self.fields.iter().map(|item| item.name.to_string()).collect();
        let mut params = Vec::new();
        for i in 1..fields.len() + 1{
            params.push(format!("${}", i))
        }
        format!("INSERT INTO {} ({}) VALUES ({});", name, fields.join(","),
            params.join(","))
    }

    fn get_fields(&self) -> String{
        let mut sb = String::new();
        for field in self.fields.as_slice(){
            sb.push_str(&Self::get_field(&field))
        }
        sb = if sb.ends_with(","){
            sb[0..sb.len() - 1].to_string()
        }else{
            sb
        };
        sb
    }

    fn get_field(field: &Field) -> String {
        let nullable = if field.required{
            " NOT NULL"
        }else{
            ""
        };
        let unique = if field.unique {
            " UNIQUE,"
        }else{
            ","
        };
        format!("\n{} {}{}{}", field.name, Self::to_sqlite(&field.datatype),
            nullable, unique)
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
