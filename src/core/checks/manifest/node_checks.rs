use crate::core::checks::common::has_description;
use crate::core::config::Config;
use crate::core::config::SpecificRuleConfig::HasDescription;
use crate::core::manifest::Manifest;

/// Applies node checks to the manifest.
///
/// # Errors
/// This function may return an error if rule `applies_to` section is missing or if rule application fails.
/// However this would never happen as default `applies_to` are set when parsing the config.
/// And config checks are done prior to this function being called.
///
/// # Panics
/// This function will panic if `applies_to` is `None` for any rule.
pub fn apply_node_checks(manifest: &Manifest, config: &Config) -> i32 {
    let mut results = Vec::new();
    for rule in &config.manifest_tests {
        for node in manifest.nodes.values() {
            if rule
                .applies_to
                .as_ref()
                .unwrap()
                .contains(&node.ruletarget())
            {
                match rule.rule {
                    HasDescription {} => {
                        let result = has_description::check_node_description(node, rule);
                        results.push(result);
                    }
                }
            }
        }
    }
    if results.iter().any(|&r| r != 0) {
        1
    } else {
        0
    }
}
