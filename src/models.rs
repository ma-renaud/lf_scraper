use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing};

#[derive(Debug, Serialize, Deserialize)]
pub struct Specie {
    pub id: Thing,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub id: Thing,
    pub specie: Specie,
    pub price: u16,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScrapedLog {
    pub id: i64,
    pub name: String,
    pub price: u16,
}
