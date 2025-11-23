use crate::core::config::{ManifestRule, Severity};
use crate::core::traits::Descriptable;
use owo_colors::OwoColorize;

pub fn print_check_row(severity: &Severity, message: &str) {
    let sev_str = severity.as_str();
    let colored_sev = match severity {
        Severity::Error => sev_str.red().bold().to_string(),
        Severity::Warning => sev_str.yellow().bold().to_string(),
    };
    println!("| {:^8} | {:<60} |", colored_sev, message);
}
pub fn check_node_description<T: Descriptable>(descriptable: &T, rule: &ManifestRule) -> i32 {
    match descriptable.description() {
        Some(desc) if !desc.trim().is_empty() => 0,
        _ => {
            let msg = format!(
                "'{}': missing description.",
                descriptable.get_object_string()
            );
            print_check_row(&rule.severity, &msg);
            1
        }
    }
}
