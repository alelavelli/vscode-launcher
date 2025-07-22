use serde_json;
use std::collections::HashMap;
use std::{error::Error, fs::File, io::Read, path::Path};
use tracing::debug;

use crate::WORKSPACE_FOLDER;
use crate::enums::ProgrammingLanguage;
use crate::model::LaunchJson;

#[tracing::instrument]
pub fn parse_launch_json(path: &Path) -> Result<LaunchJson, Box<dyn Error>> {
    debug!("Parsing launch json file");

    let mut file = File::open(path.join(".vscode/launch.json"))?;
    let mut file_string = String::new();
    debug!("Reading file");
    file.read_to_string(&mut file_string)?;

    debug!("File read, start cleaning of the raw string");
    let mut cleaned_string = String::new();
    // clear the string removing any line that starts with // which is a comment
    for line in file_string.lines() {
        if !line.trim_ascii_start().starts_with("//") {
            // Replace the workspace placeholder with the actual path
            cleaned_string.push_str(&line.replace(WORKSPACE_FOLDER, path.to_str().unwrap())); //path.to_str().unwrap()));
        }
    }

    debug!("File cleaned, start parsing raw string to LaunchJson struct");
    let mut launch_json: LaunchJson = serde_json::from_str(&cleaned_string)?;

    debug!(
        "Setting cwd attribute with path as default and defining the configuration programming language."
    );
    // Set cwd as the given path if it is missing in the configuration
    for configuration in launch_json.configurations.iter_mut() {
        if configuration.cwd.is_none() {
            configuration.cwd = Some(".".to_string());
        }

        match configuration.config_type.as_str() {
            "debugpy" => configuration.programming_language = ProgrammingLanguage::Python,
            "node-terminal" => configuration.programming_language = ProgrammingLanguage::JavaScript,
            "lldb" => configuration.programming_language = ProgrammingLanguage::Rust,
            _ => configuration.programming_language = ProgrammingLanguage::Unknown,
        }
    }

    debug!(?launch_json, "Parsing completed");
    Ok(launch_json)
}

#[tracing::instrument]
pub fn parse_dotenv(path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut result = HashMap::new();
    let mut file = File::open(path)?;
    let mut env_file_string = String::new();
    file.read_to_string(&mut env_file_string)?;

    for line in env_file_string.lines() {
        let mut parts = line.splitn(2, '=');
        let key = parts.next().unwrap_or("").trim();
        let mut value = parts.next().unwrap_or("").trim();
        if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
            value = &value[1..value.len() - 1];
        }

        if !key.is_empty() {
            result.insert(key.into(), value.into());
        }
    }
    Ok(result)
}
