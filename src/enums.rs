use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Enumeration of the supported programming language by vscl
///
/// When it cannot infer the language, then Unknown is used
#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ProgrammingLanguage {
    JavaScript,
    Python,
    Rust,
    #[default]
    Unknown,
}

impl Display for ProgrammingLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProgrammingLanguage::JavaScript => "JavaScript",
                ProgrammingLanguage::Python => "Python",
                ProgrammingLanguage::Rust => "Rust",
                ProgrammingLanguage::Unknown => "Unknown",
            }
        )
    }
}
