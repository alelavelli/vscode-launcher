use tracing::debug;

use crate::model::Configuration;
use crate::parsing::parse_dotenv;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[tracing::instrument]
pub fn run_config(configuration: Configuration) -> Result<(), Box<dyn Error>> {
    // We use ctrlc create to gracefully stop on ctrl-c SIGINT signal
    // To do that, we use a running boolean variable used to determine the
    // close of the program
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Set the handler, when ctrl-c is pressed then we set the variable to false
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process");
        Ok(())
    } else {
        debug!("Executing in linux os");
        let python = configuration.python.unwrap();
        let program = configuration.program.unwrap();

        debug!("Launching command: {} {}", python, program);

        let mut command = Command::new("sh");

        // Set as current working directory the workspace folder
        if let Some(cwd) = &configuration.cwd {
            debug!(cwd, "Setting current working directory");
            command.current_dir(cwd);
        }

        // Set program to launch via python interpreter
        command.arg("-c").arg(format!("{} {}", python, program));

        // Set environment variables
        if let Some(env_vars) = configuration.env {
            command.envs(env_vars);
        }

        // If envFile is specified then we open the .env file and load them as environment variables
        if let Some(env_file_path) = configuration.env_file {
            let base_path = if let Some(cwd) = &configuration.cwd {
                Path::new(cwd)
            } else {
                Path::new("")
            };

            let env_file_vars = parse_dotenv(&base_path.join(env_file_path))?;
            command.envs(env_file_vars);
        }

        debug!("Preparing stdout and stderr channels");
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
            for line in stdout_reader.lines().map_while(Result::ok) {
                println!("{}", line);
            }
        });

        let stderr_handle = std::thread::spawn(move || {
            for line in stderr_reader.lines().map_while(Result::ok) {
                println!("{}", line);
            }
        });

        // When the child process stops we can close the threads
        debug!("Command in execution, waiting it to complete.");
        // loop until the running variable is set to false
        // in this case, kill the child process and close the threads
        child.try_wait().unwrap();
        while running.load(Ordering::SeqCst) {
            child.try_wait().unwrap();
        }
        child.kill().unwrap();
        debug!("Command completed, closing stdout and stderr threads");
        stdout_handle.join().unwrap();
        stderr_handle.join().unwrap();
        debug!("Execution completed, closing.");
        Ok(())
    }
}
