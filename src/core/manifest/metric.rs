use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct MetricDependsOn {
//     pub macros: Option<Vec<String>>,
//     pub nodes: Option<Vec<String>>,
// }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Metric {
    pub name: String,
    // pub package_name: String,
    // pub path: String,
    // pub original_file_path: String,
    // pub unique_id: String,
    // pub fqn: Vec<String>,
    // pub description: String,
    // pub label: String,
    // #[serde(rename = "type")]
    // pub metric_type: String,
    // pub type_params: serde_json::Value,
    // pub filter: Option<serde_json::Value>,
    // pub metadata: Option<serde_json::Value>,
    // pub time_granularity: Option<String>,
    // pub meta: Option<serde_json::Value>,
    // pub tags: Option<Vec<String>>,
    // pub config: Option<serde_json::Value>,
    // pub unrendered_config: Option<serde_json::Value>,
    // pub sources: Option<Vec<serde_json::Value>>,
    // pub depends_on: Option<MetricDependsOn>,
    // pub refs: Option<Vec<serde_json::Value>>,
    // pub metrics: Option<Vec<serde_json::Value>>,
    // pub created_at: Option<f64>,
    // pub group: Option<String>,
}
