use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub id: Option<i64>,
    pub name: String,
    pub path: String,
    pub is_enabled: bool,
    pub method: String,
}