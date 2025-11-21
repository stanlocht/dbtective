use crate::core::checks::RuleResult;
use crate::core::config::ManifestRule;
use crate::core::dbt_objects::nodes::node::Node;

pub fn check_node_description(node: &Node, rule: &ManifestRule) -> RuleResult {
    let (passed, message) = match node.get_description() {
        Some(desc) if !desc.trim().is_empty() => (true, String::new()),
        _ => (
            false,
            format!("'{}' is missing a description.", node.original_file_path()),
        ),
    };

    RuleResult {
        severity: rule.severity.clone(),
        code: i32::from(!passed),
        node_type: node.ruletarget().to_string(),
        message,
    }
}
