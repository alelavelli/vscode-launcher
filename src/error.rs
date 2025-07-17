use std::fmt::Display;

/// Error type returned by the runner module
#[derive(Debug)]
pub enum RunnerError {
    UnknownProgrammingLanguage,
    InvalidConfiguration(String),
    GenericError(String),
}

impl Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RunnerError::UnknownProgrammingLanguage =>
                    "Programming language must be known to launch the configuration".to_string(),
                RunnerError::InvalidConfiguration(message) => message.clone(),
                RunnerError::GenericError(message) => format!("Got error: {message}"),
            }
        )
    }
}
