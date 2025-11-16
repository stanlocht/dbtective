use super::super::column::Column;
use super::super::{Meta, Tags};
// use super::super::NodeDocs
use super::{Analysis, HookNode, Model, Seed, Snapshot, SqlOperation, Test};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(tag = "resource_type")]
#[allow(dead_code)]
pub enum Node {
    #[serde(rename = "analysis")]
    Analysis(Analysis),
    #[serde(rename = "seed")]
    Seed(Seed),
    #[serde(rename = "model")]
    Model(Model),
    #[serde(rename = "test")]
    Test(Test),
    #[serde(rename = "snapshot")]
    Snapshot(Snapshot),
    #[serde(rename = "operation")]
    HookNode(HookNode),
    #[serde(rename = "sql_operation")]
    SqlOperation(SqlOperation),
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FileHash {
    pub name: String,
    pub checksum: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MacroDependsOn {
    pub macros: Vec<String>,
}

// Base Layer: Core fields ALL nodes have
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NodeBase {
    pub database: Option<String>,
    pub schema: String,
    pub name: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    pub alias: String,
    pub checksum: FileHash,

    // Common optional fields
    pub tags: Option<Tags>,
    pub description: Option<String>,
    pub meta: Option<Meta>,
    pub columns: Option<HashMap<String, Column>>,
    pub config: Option<serde_json::Value>,
    // Currently unused fields that do exist in the data
    // pub group: Option<String>,
    // pub docs: Option<NodeDocs>,
    // pub patch_path: Option<String>,
    // pub build_path: Option<String>,
    // pub unrendered_config: Option<serde_json::Value>,
    // pub created_at: Option<f64>,
    // pub config_call_dict: Option<serde_json::Value>,
    // pub unrendered_config_call_dict: Option<serde_json::Value>,
    // pub relation_name: Option<String>,
    // pub raw_code: Option<String>,
    // pub doc_blocks: Option<Vec<String>>,
    // pub root_path: Option<String>,
    // pub depends_on: Option<MacroDependsOn>,
}

// Layer 2: Compiled node specific fields
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CompiledNodeFields {
    pub language: Option<String>,
    // Currently unused fields that do exist in the data
    // pub refs: Option<Vec<serde_json::Value>>,
    // pub sources: Option<Vec<serde_json::Value>>,
    // pub metrics: Option<Vec<serde_json::Value>>,
    // pub compiled_path: Option<String>,
    // pub compiled: Option<bool>,
    // pub compiled_code: Option<String>,
    // pub extra_ctes_injected: Option<bool>,
    // pub extra_ctes: Option<Vec<serde_json::Value>>,
    // #[serde(rename = "_pre_injected_sql")]
    // pub pre_injected_sql: Option<String>,
    // pub contract: Option<serde_json::Value>,
}
