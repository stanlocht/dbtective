use super::super::column::Column;
use super::node::NodeBase;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Seed {
    #[serde(flatten)]
    pub base: NodeBase,
    config: Option<serde_json::Value>, // placeholder
    columns: HashMap<String, Column>,
    defer_relation: Option<serde_json::Value>, // placeholder
}
