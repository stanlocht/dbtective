mod cli;
mod core;
use crate::cli::commands::{Cli, Commands};
use crate::core::checks::manifest::node_checks::apply_node_checks;
use crate::core::config::Config;
use crate::core::manifest::Manifest;
use clap::{CommandFactory, Parser};
use log::{debug, info};
use owo_colors::OwoColorize;
use std::process::exit;
use std::time::Instant;
use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct RuleResultDisplay {
    status: &'static str,
    node_type: String,
    message: String,
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Run { options }) => {
            if args.verbose {
                debug!("Starting dbtective analysis...");
                debug!("{options:#?}");
            }

            let start = Instant::now();
            let manifest_path = std::path::PathBuf::from(format!(
                "{}/{}",
                options.entry_point, options.manifest_file
            ));

            let manifest = match Manifest::from_file(&manifest_path) {
                Ok(manifest) => manifest,
                Err(err) => {
                    eprintln!("{}", err.to_string().red());
                    exit(1);
                }
            };
            let config =
                match Config::from_file(format!("{}/{}", options.entry_point, options.config_file))
                {
                    Ok(cfg) => cfg,
                    Err(err) => {
                        eprintln!("{}", err.to_string().red());
                        exit(1);
                    }
                };

            let results = apply_node_checks(&manifest, &config);
            if results.is_empty() {
                println!("{}", "All checks passed successfully!".green());
            } else {
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

                let mut table = Table::new(display_rows);
                table.with(Style::modern());
                println!("{}", table);
            }

            let duration = start.elapsed();
            println!("Analysis completed in: {:?}", duration);
        }
        Some(Commands::Init { options }) => {
            if args.verbose {
                debug!("Initializing dbtective project...");
                debug!("{options:#?}");
            }
            // Initialization logic here
        }
        None => {
            info!(
                "\n {}",
                r"
                ██████╗ ██████╗ ████████╗███████╗ ██████╗████████╗██╗██╗   ██╗███████╗
                ██╔══██╗██╔══██╗╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██║██║   ██║██╔════╝
                ██║  ██║██████╔╝   ██║   █████╗  ██║        ██║   ██║██║   ██║█████╗
                ██║  ██║██╔══██╗   ██║   ██╔══╝  ██║        ██║   ██║╚██╗ ██╔╝██╔══╝
                ██████╔╝██████╔╝   ██║   ███████╗╚██████╗   ██║   ██║ ╚████╔╝ ███████╗
                ╚═════╝ ╚═════╝    ╚═╝   ╚══════╝ ╚═════╝   ╚═╝   ╚═╝  ╚═══╝  ╚══════╝

                "
            );
            info!(
                "{}",
                "\t \t 🕵️ \t dbtective - On the case for your dbt best practices! \t 🕵️ \n".red()
            );
            Cli::command().print_help().unwrap();
        }
    }
}
