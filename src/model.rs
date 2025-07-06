use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchJson {
    pub version: String,
    pub configurations: Vec<Configuration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    #[serde(rename = "type")]
    pub config_type: String,
    pub request: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(rename = "envFile", skip_serializing_if = "Option::is_none")]
    pub env_file: Option<String>,
}
