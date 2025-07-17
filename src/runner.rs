use tracing::debug;

use crate::enums::ProgrammingLanguage;
use crate::error::RunnerError;
use crate::model::Configuration;
use crate::parsing::parse_dotenv;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn configure_python(
    configuration: &Configuration,
    command: &mut Command,
) -> Result<(), RunnerError> {
    let python = configuration
        .python
        .as_ref()
        .ok_or(RunnerError::InvalidConfiguration(
            "In Python configuration, python field must be present".into(),
        ))?;

    let mut program = configuration
        .program
        .as_ref()
        .ok_or(RunnerError::InvalidConfiguration(
            "In Python configuration, program field must be present".into(),
        ))?
        .clone();

    debug!("Launching command: {} {}", python, program);

    // Set program to launch via python interpreter
    if let Some(args_value) = configuration.args.as_ref() {
        //let mut args: Vec<String> = vec![];
        if let Some(args_vec_value) = args_value.as_array() {
            for elem in args_vec_value {
                if let Some(string_args) = elem.as_str() {
                    program.push(' ');
                    program.push_str(string_args);
                }
            }
        }
    }
    command.arg(format!("{} {}", python, program));
    Ok(())
}

fn configure_javascript(
    _configuration: &Configuration,
    _command: &mut Command,
) -> Result<(), RunnerError> {
    todo!()
}

fn configure_rust(
    _configuration: &Configuration,
    _command: &mut Command,
) -> Result<(), RunnerError> {
    todo!()
}

/// Run the configuration
///
/// It builds the command step by step adding the values present in the configuration.
///
/// - The actual command arguments are defined by the programming language
/// - If cwd is present, then we set it so that the program will be run in the right place.
/// - If args is present, then we extract string values and append them to the program string.
///   They are arguments of the python program and not of the current command, therefore, we
///   put them into the program string
/// - if env is present, then we add them as environment variables of the command
/// - if env_file is present, then we load it, extract the variables and add them as environment
///   variables of the command
#[tracing::instrument]
pub fn run_config(configuration: Configuration) -> Result<(), RunnerError> {
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

    let mut command = if cfg!(target_os = "windows") {
        let mut command = Command::new("cmd");
        command.arg("/C");
        command
    } else {
        debug!("Executing in linux os");
        let mut command = Command::new("sh");
        command.arg("-c");
        command
    };

    match configuration.programming_language {
        ProgrammingLanguage::Python => configure_python(&configuration, &mut command)?,
        ProgrammingLanguage::JavaScript => configure_javascript(&configuration, &mut command)?,
        ProgrammingLanguage::Rust => configure_rust(&configuration, &mut command)?,
        ProgrammingLanguage::Unknown => return Err(RunnerError::UnknownProgrammingLanguage),
    }

    // Set as current working directory the workspace folder
    if let Some(cwd) = &configuration.cwd {
        debug!(cwd, "Setting current working directory");
        command.current_dir(cwd);
    }

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

        let env_file_vars = parse_dotenv(&base_path.join(env_file_path))
            .map_err(|e| RunnerError::GenericError(e.to_string()))?;
        command.envs(env_file_vars);
    }

    debug!("Preparing stdout and stderr channels");
    // Attach stdout and stderr
    command.stdout(Stdio::piped()).stderr(Stdio::piped());

    // Launch the process and get the handler
    debug!("Ready to start command {:?}", command.get_args());
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
        let result = child.try_wait().unwrap();
        if result.is_some() {
            debug!("Got {:?}", result);
            running.store(false, Ordering::SeqCst);
        }
    }
    child.kill().unwrap();
    debug!("Command completed, closing stdout and stderr threads");
    stdout_handle.join().unwrap();
    stderr_handle.join().unwrap();
    debug!("Execution completed, closing.");
    Ok(())
}
