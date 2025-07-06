use crate::model::Configuration;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn run_config(configuration: Configuration) {
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

        // Set as current working directory the workspace folder
        if let Some(cwd) = configuration.cwd {
            command.current_dir(cwd);
        }

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
