use clap::{Parser, Subcommand};
use vscode_launcher::model::Configuration;

use std::path::Path;

use vscode_launcher::parsing::parse_launch_json;
use vscode_launcher::runner::run_config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Turn on debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Lists the available launch configs in the given workspace
    Ls {
        /// Path of a vscode workspace that contains .vscode folder with launch.json file
        #[arg(short, long)]
        workspace: String,
    },
    /// Runs the configuration in the workspace
    Run {
        /// Path of a vscode workspace that contains .vscode folder with launch.json file
        #[arg(short, long)]
        workspace: String,
        /// Name of the configuration in the launch.json configuration file
        #[arg(short, long)]
        name: String,
    },
}

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

fn main() {
    let cli = Cli::parse();

    let tracing_level = match cli.debug {
        0 => tracing::Level::ERROR,
        1 => tracing::Level::WARN,
        2 => tracing::Level::INFO,
        3 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    tracing_subscriber::fmt()
        .with_max_level(tracing_level)
        .init();

    match &cli.command {
        Commands::Ls { workspace } => {
            let result = parse_launch_json(Path::new(&workspace));
            if result.is_err() {
                println!(
                    "Ops, an error has been occurred. Got {}",
                    result.err().unwrap()
                );
            } else {
                let launch_json = result.unwrap();
                println!("The workspace {workspace} contains the following configurations:");
                for config in launch_json.configurations {
                    println!("    - {}", config.name);
                }
            }
        }
        Commands::Run { workspace, name } => {
            let result = parse_launch_json(Path::new(&workspace));
            if result.is_err() {
                println!(
                    "Ops, an error has been occurred. Got {}",
                    result.err().unwrap()
                );
            } else {
                let launch_json = result.unwrap();
                let filter_configuration: Vec<Configuration> = launch_json
                    .configurations
                    .into_iter()
                    .filter(|config| &config.name == name)
                    .collect();

                if filter_configuration.is_empty() {
                    println!(
                        "Configuration `{name}` is not present in the workspace `{workspace}`"
                    );
                } else if filter_configuration.len() > 1 {
                    println!(
                        "Found more than one configuration with name `{name}` for workspace `{workspace}`"
                    )
                } else {
                    println!("Ready to run configuration {name}");
                    run_config(filter_configuration.into_iter().next().unwrap());
                }
            }
        }
    }
}
