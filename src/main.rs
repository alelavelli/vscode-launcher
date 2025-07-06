use std::path::Path;

use tracing::info;
use vscode_launcher::parsing::parse_launch_json;
use vscode_launcher::runner::run_config;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let result = parse_launch_json(&Path::new("WORKSPACE_PATH"));
    for config in result.unwrap().configurations {
        if config.name == "CONFIG_NAME" {
            info!("Config with name {}\n{:?}", config.name, config);
            run_config(config);
        }
    }
}
