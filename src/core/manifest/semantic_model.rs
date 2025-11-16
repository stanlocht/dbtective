use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct SemanticModelDependsOn {
//     pub macros: Vec<String>,
//     pub nodes: Vec<String>,
// }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SemanticModel {
    pub name: String,
    // pub resource_type: String,
    // pub path: String,
    // pub original_file_path: String,
    // pub unique_id: String,
    // pub fqn: Vec<String>,
    // pub model: String,
    // pub node_relation: serde_json::Value,
    // pub description: Option<String>,
    // pub label: Option<String>,
    // pub defaults: Option<serde_json::Value>,
    // pub entities: Option<Vec<serde_json::Value>>,
    // pub measures: Option<Vec<serde_json::Value>>,
    // pub dimensions: Option<Vec<serde_json::Value>>,
    // pub metadata: Option<serde_json::Value>,
    // pub depends_on: Option<SemanticModelDependsOn>,
    // pub refs: Option<Vec<serde_json::Value>>,
    // pub created_at: Option<f64>,
    // pub config: Option<serde_json::Value>,
    // pub unrendered_config: Option<serde_json::Value>,
    // pub primary_entity: Option<String>,
    // pub group: Option<String>,
}
