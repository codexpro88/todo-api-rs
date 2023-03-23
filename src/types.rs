use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoItem {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub description: String,
    pub time_created: String,
}

