use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field{
    pub name: String,
    pub datatype: String,
    pub label: String,
    pub placeholder: String,
    pub required: bool,
    pub unique: bool,
}

