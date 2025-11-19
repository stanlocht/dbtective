use crate::core::checks::RuleResult;
use crate::core::config::ManifestRule;
use crate::core::dbt_objects::nodes::node::Node;
use anyhow::Result;

pub fn check_node_description(node: &Node, rule: &ManifestRule) -> Result<RuleResult> {
    let (passed, message) = match node.get_description() {
        Some(desc) if !desc.trim().is_empty() => (true, String::new()),
        _ => (
            false,
            format!("'{}' is missing a description.", node.original_file_path()),
        ),
    };

    Ok(RuleResult {
        severity: rule.severity.clone(),
        code: !passed as i32,
        node_type: node.ruletarget().to_string(),
        message,
    })
}
