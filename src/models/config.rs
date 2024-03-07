use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_yaml::Error;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration{
    pub log_level: String,
    pub db_url: String,
    pub port: String,
    pub username: String,
    pub password: String,
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
}



