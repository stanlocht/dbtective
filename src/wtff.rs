use tabled::{
    settings::{object::Columns, Alignment, Highlight, Segment, Style},
    Table, Tabled,
};

#[derive(Tabled)]
struct RuleResultDisplay {
    status: &'static str,
    node_type: String,
    message: String,
}

fn main() {
    // ...existing code...

    let display_rows: Vec<RuleResultDisplay> = results
        .into_iter()
        .filter_map(|result| match result {
            Ok(rule_result) if rule_result.code != 0 => Some(RuleResultDisplay {
                status: rule_result.severity.as_str(),
                node_type: rule_result.node_type,
                message: rule_result.message,
            }),
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                None
            }
            _ => None,
        })
        .collect();

    if !display_rows.is_empty() {
        let mut table = Table::new(display_rows);
        table.with(Style::modern());
        table.modify(Columns::first(), Alignment::center());
        table.with(Highlight::outline(Segment::all(), '*')); // highlight outer border with '*'
        println!("{}", table);
    }

    // ...rest of your code...
}
