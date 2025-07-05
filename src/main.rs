use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::{collections::HashMap, error::Error, fs::File, io::Read, path::Path};

static WORKSPACE_FOLDER: &str = "${workspaceFolder}";

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchJson {
    pub version: String,
    pub configurations: Vec<Configuration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    #[serde(rename = "type")]
    pub config_type: String,
    pub request: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub python: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
}

fn parse_launch_json(path: &Path) -> Result<LaunchJson, Box<dyn Error>> {
    let mut file = File::open(path.join(".vscode/launch.json"))?;
    let mut file_string = String::new();
    file.read_to_string(&mut file_string)?;

    let mut clean_string = String::new();
    // clear the string removing any line that starts with // which is a comment
    for line in file_string.lines() {
        if !line.trim_ascii_start().starts_with("//") {
            // Replace the workspace placeholder with the actual path
            clean_string.push_str(&line.replace(WORKSPACE_FOLDER, path.to_str().unwrap()));
        }
    }

    let launch_json = serde_json::from_str(&clean_string)?;

    Ok(launch_json)
}

fn run_config(configuration: Configuration) {
    let _ = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process");
    } else {
        let python = configuration.python.unwrap();
        let program = configuration.program.unwrap();

        println!("Launching command: {} {}", python, program);

        let mut command = Command::new("sh");

        // Set program to launch via python interpreter
        command.arg("-c").arg(format!("{} {}", python, program));

        // Set environment variables
        if let Some(env_vars) = configuration.env {
            command.envs(env_vars);
        }

        // Attach stdout and stderr
        command.stdout(Stdio::piped()).stderr(Stdio::piped());

        // Launch the process and get the handler
        let mut child = command.spawn().expect("failed to execute process");

        /*
        To print live the stdout and stderr we need to take out the handlers and spawn two
        threads with a BufReader that print anytime the buffer get a value
        */
        let stdout = child.stdout.take().expect("failed to get stdout");
        let stderr = child.stderr.take().expect("failed to get stderr");

        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        let stdout_handle = std::thread::spawn(move || {
            for line in stdout_reader.lines() {
                if let Ok(line) = line {
                    println!("{}", line);
                }
            }
        });

        let stderr_handle = std::thread::spawn(move || {
            for line in stderr_reader.lines() {
                if let Ok(line) = line {
                    eprintln!("{}", line);
                }
            }
        });

        // When the child process stops we can close the threads
        child.wait().unwrap();
        stdout_handle.join().unwrap();
        stderr_handle.join().unwrap();
    };
}

fn main() {
    let result = parse_launch_json(&Path::new("PATH_GOES_HERE"));
    for config in result.unwrap().configurations {
        if config.name == "TARGET_CONFIGURATION_NAME_GOES_HERE" {
            println!("Config with name {}\n{:?}", config.name, config);
            run_config(config);
        }
    }
}
