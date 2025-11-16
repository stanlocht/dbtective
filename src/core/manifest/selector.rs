use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Selector {
    pub name: String,
    pub default: Option<bool>,
    pub definition: serde_json::Value,
}
