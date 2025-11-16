use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct UnitTestDependsOn {
//     pub macros: Vec<String>,
//     pub nodes: Vec<String>,
// }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UnitTest {
    pub name: String,
    pub model: String,
    // pub given: Vec<serde_json::Value>,
    // pub expect: serde_json::Value,
    // pub resource_type: String,
    // pub package_name: String,
    // pub path: String,
    // pub original_file_path: String,
    // pub unique_id: String,
    // pub fqn: Vec<String>,
    // pub description: Option<String>,
    // pub overrides: Option<serde_json::Value>,
    // pub depends_on: Option<UnitTestDependsOn>,
    // pub config: Option<serde_json::Value>,
    // pub checksum: Option<String>,
    // pub schema: Option<String>,
    // pub created_at: Option<f64>,
    // pub versions: Option<serde_json::Value>,
    // pub version: Option<serde_json::Value>,
}
