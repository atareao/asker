use chrono::{
    DateTime,
    Utc,
};

pub fn default_datetime() -> DateTime<Utc>{
    Utc::now()
}

