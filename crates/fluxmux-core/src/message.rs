use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Format {
    Json,
    Csv,
    Yaml,
    Avro,
    Parquet,
    Text,
    Binary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Option<String>,
    pub key: Option<Vec<u8>>,
    pub payload: Vec<u8>,              // raw bytes
    pub format: Option<Format>,
    pub parsed: Option<serde_json::Value>, // cached parsed JSON
    pub timestamp: DateTime<Utc>,
    pub headers: HashMap<String, String>,
    pub meta: HashMap<String, String>, // connector-specific metadata
}
