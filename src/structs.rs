use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct HistoryImage {
    pub filename: String,
    pub subfolder: String,
    #[serde(rename = "type")]
    pub image_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HistoryOutput {
    pub images: Vec<HistoryImage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HistoryData {
    pub outputs: HashMap<String, HistoryOutput>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerationResponse {
    pub prompt_id: String,
    pub number: i128,
    pub node_errors: BTreeMap<String, serde_json::Value>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GenerationNodeError {}

#[derive(Clone)]
pub struct ComfyUI {
    pub client_id: Uuid,
    pub url: String,
    pub port: i32,
}
impl ComfyUI {
    pub fn format_url(self) -> String {
        format!("{}:{}", self.url, self.port)
    }
}
