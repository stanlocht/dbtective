use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct MacroDependsOn {
//     pub macros: Vec<String>,
// }

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct MacroArgument {
//     pub name: String,
//     #[serde(rename = "type")]
//     pub arg_type: Option<String>,
//     pub description: Option<String>,
// }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Macro {
    pub name: String,
    // pub package_name: String,
    // pub path: String,
    // pub original_file_path: String,
    // pub unique_id: String,
    // pub macro_sql: String,
    // pub depends_on: MacroDependsOn,
    // pub description: Option<String>,
    // pub meta: Option<serde_json::Value>,
    // pub docs: Option<serde_json::Value>,
    // pub patch_path: Option<String>,
    // pub arguments: Option<Vec<MacroArgument>>,
    // pub created_at: Option<f64>,
    // pub supported_languages: Option<Vec<String>>,
}
