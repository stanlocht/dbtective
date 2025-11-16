use super::super::{Docs, Meta, Tags};
use super::{Analysis, HookNode, Model, Seed, Snapshot, SqlOperation, Test};
use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NodeBase {
    // Required fields
    pub database: Option<String>,
    pub schema: String,
    pub name: String,
    // pub resource_type: String, Used for enum tagging, not incorporated here
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub fqn: Vec<String>,
    pub alias: String,
    pub checksum: FileHash,

    // General fields we will use
    tags: Option<Tags>,
    description: Option<String>,
    meta: Option<Meta>,

    // Optional fields we might use later
    group: Option<String>,
    docs: Option<Docs>,
    patch_path: Option<String>,
    build_path: Option<String>,
    unrendered_config: Option<serde_json::Value>,
    created_at: Option<f64>, // e.g.  "created_at": 1755247538.5673869,
    config_call_dict: Option<serde_json::Value>,
    relation_name: Option<String>,
    raw_code: Option<String>,
    doc_blocks: Option<Vec<String>>,
    root_path: Option<String>,
    depends_on: Option<MacroDependsOn>,
}
