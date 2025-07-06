use serde_json;
use std::{error::Error, fs::File, io::Read, path::Path};

use crate::WORKSPACE_FOLDER;
use crate::model::LaunchJson;

pub fn parse_launch_json(path: &Path) -> Result<LaunchJson, Box<dyn Error>> {
    let mut file = File::open(path.join(".vscode/launch.json"))?;
    let mut file_string = String::new();
    file.read_to_string(&mut file_string)?;

    let mut cleaned_string = String::new();
    // clear the string removing any line that starts with // which is a comment
    for line in file_string.lines() {
        if !line.trim_ascii_start().starts_with("//") {
            // Replace the workspace placeholder with the actual path
            cleaned_string.push_str(&line.replace(WORKSPACE_FOLDER, path.to_str().unwrap()));
        }
    }

    let mut launch_json: LaunchJson = serde_json::from_str(&cleaned_string)?;

    // Set cwd as the given path if it is missing in the configuration
    for configuration in launch_json.configurations.iter_mut() {
        if configuration.cwd.is_none() {
            configuration.cwd = Some(path.to_str().unwrap().into());
        }
    }

    Ok(launch_json)
}
