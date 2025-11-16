use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SavedQueryDependsOn {
    pub macros: Option<Vec<String>>,
    pub nodes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SavedQuery {
    pub name: String,
    // pub resource_type: String,
    // pub package_name: String,
    // pub path: String,
    // pub original_file_path: String,
    // pub unique_id: String,
    // pub fqn: Vec<String>,
    // pub query_params: serde_json::Value,
    // pub exports: Vec<serde_json::Value>,
    // pub description: Option<String>,
    // pub label: Option<String>,
    // pub metadata: Option<serde_json::Value>,
    // pub config: Option<serde_json::Value>,
    // pub unrendered_config: Option<serde_json::Value>,
    // pub group: Option<String>,
    // pub depends_on: Option<SavedQueryDependsOn>,
    // pub created_at: Option<f64>,
    // pub refs: Option<Vec<serde_json::Value>>,
    // pub tags: Option<Vec<String>>,
}
