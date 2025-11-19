use anyhow::{Context, Result};
use serde::Deserialize;
use std::fmt;
use std::fs::File;
use std::path::Path;

const ALL_RULE_TARGETS: &[RuleTarget] = &[
    RuleTarget::Models,
    RuleTarget::Seeds,
    RuleTarget::Sources,
    RuleTarget::Macros,
    RuleTarget::Metrics,
    RuleTarget::Exposures,
    RuleTarget::SemanticModels,
    RuleTarget::SavedQueries,
    RuleTarget::Tests,
    RuleTarget::Analyses,
    RuleTarget::Snapshots,
    RuleTarget::HookNodes,
];

fn default_applies_to_for_rule(rule_type: &SpecificRuleConfig) -> Vec<RuleTarget> {
    match rule_type {
        SpecificRuleConfig::HasDescription {} => ALL_RULE_TARGETS.to_vec(),
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuleTarget {
    Models,
    Seeds,
    Sources,
    Macros,
    Metrics,
    Exposures,
    SemanticModels,
    SavedQueries,
    Tests,
    Analyses,
    Snapshots,
    HookNodes,
}
impl fmt::Display for RuleTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let singular = match self {
            RuleTarget::Models => "Model",
            RuleTarget::Seeds => "Seed",
            RuleTarget::Sources => "Source",
            RuleTarget::Macros => "Macro",
            RuleTarget::Metrics => "Metric",
            RuleTarget::Exposures => "Exposure",
            RuleTarget::SemanticModels => "SemanticModel",
            RuleTarget::SavedQueries => "SavedQuery",
            RuleTarget::Tests => "Test",
            RuleTarget::Analyses => "Analysis",
            RuleTarget::Snapshots => "Snapshot",
            RuleTarget::HookNodes => "HookNode",
        };
        write!(f, "{}", singular)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}
impl Severity {
    #[allow(dead_code)]
    pub fn as_code(&self) -> u8 {
        match self {
            Severity::Error => 1,
            Severity::Warning => 0,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Error => "FAIL",
            Severity::Warning => "WARN",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum SpecificRuleConfig {
    #[serde(rename = "has_description")]
    HasDescription {},
}

fn default_severity() -> Severity {
    Severity::Error
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ManifestRule {
    pub name: String,
    #[serde(default = "default_severity")]
    pub severity: Severity,
    pub description: Option<String>,
    pub applies_to: Option<Vec<RuleTarget>>,
    #[serde(flatten)]
    pub rule: SpecificRuleConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub manifest_tests: Vec<ManifestRule>,
}
impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let file = File::open(path)
            .context(format!("Unable to open config file at {}", path.display()))?;

        let mut config: Config =
            serde_yaml::from_reader(file).context("Failed to parse configuration")?;

        config.apply_default_applies_to();
        Ok(config)
    }

    pub fn apply_default_applies_to(&mut self) {
        for rule in &mut self.manifest_tests {
            if rule.applies_to.is_none() {
                rule.applies_to = Some(default_applies_to_for_rule(&rule.rule));
            }
        }
    }
}

#[cfg(test)]
fn create_temp_file_from_str(content: &str) -> tempfile::NamedTempFile {
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    std::io::Write::write(&mut temp_file, content.as_bytes()).unwrap();
    temp_file
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let simple_rule = r#"
manifest_tests:
  - name: "model_seeds_have_description"
    type: "has_description"
    severity: "error"
    description: "All nodes must have a description."
"#;
        let temp_file = create_temp_file_from_str(simple_rule);
        let config = Config::from_file(temp_file.path()).expect("Failed to parse config");

        println!("{:#?}", config);

        assert_eq!(config.manifest_tests.len(), 2);
        assert_eq!(
            config.manifest_tests[0].name,
            "model_seeds_have_description"
        );
        assert_eq!(config.manifest_tests[0].severity, Severity::Error);
        assert_eq!(config.manifest_tests[1].name, "columns_are_lowercase");
    }

    #[test]
    fn test_default_applies_to_for_rule() {
        let applies_to = default_applies_to_for_rule(&SpecificRuleConfig::HasDescription {});
        assert_eq!(applies_to, vec![RuleTarget::Models, RuleTarget::Seeds]);
    }
}
