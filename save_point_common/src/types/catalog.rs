use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Catalog {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub namespace: String,
    pub entry: HashMap<String, CatalogItem>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CatalogItem {
    pub unique: String,
    pub artifact: String,
    pub receive_at: DateTime<Utc>,
}

impl Catalog {
    pub const FILE_NAME: &str = "catalog.yaml";
    pub const URI_SCHEME: &str = "catalog.sp";
}
