use std::path::Path;

use vscode_launcher::parsing::parse_launch_json;
use vscode_launcher::runner::run_config;

fn main() {
    let result = parse_launch_json(&Path::new("PATH_GOES_HERE"));
    for config in result.unwrap().configurations {
        if config.name == "TARGET_CONFIGURATION_NAME_GOES_HERE" {
            println!("Config with name {}\n{:?}", config.name, config);
            run_config(config);
        }
    }
}
