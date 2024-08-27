//! **Piston Client Library Data Structures**
//!
//! This module defines several data structures used by the Piston client library to represent languages, API responses, and execution results.

#[derive(Clone, Debug, Deserialize)]
/// Represents a programming language supported by Piston.
pub struct Language {
    /// The name of the programming language.
    pub language: String,
    /// The version of the language.
    pub version: String,
    /// A list of aliases for the language name.
    pub aliases: Vec<String>,
    /// The optional runtime used to execute code in this language.
    pub runtime: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
/// Represents the possible responses from the Piston API.
pub enum ApiResponse {
    /// A successful execution response containing execution data.
    Good(Response),
    /// An error response containing an error message.
    Error(ErrorResponse),
}

#[derive(Debug, Deserialize)]
/// Represents the data returned by a successful Piston execution.
pub struct Response {
    /// The programming language used for execution.
    pub language: String,
    /// The version of the language used.
    pub version: String,
    /// The execution data containing output and other details.
    pub run: RunData,
}

impl Response {
    /// Returns a reference to the language used for execution.
    #[must_use]
    pub fn language(&self) -> &str {
        self.language.as_ref()
    }

    /// Returns a reference to the version of the language used.
    #[must_use]
    pub fn version(&self) -> &str {
        self.version.as_ref()
    }

    /// Returns a reference to the execution data.
    #[must_use]
    pub fn data(&self) -> &RunData {
        &self.run
    }
}

#[derive(Debug, Deserialize)]
/// Represents the error information returned by the Piston API.
pub struct ErrorResponse {
    /// The error message describing the issue.
    pub message: String,
}

impl ErrorResponse {
    /// Returns a reference to the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}

#[derive(Debug, Deserialize)]
/// Represents the data associated with a Piston execution run.
pub struct RunData {
    /// The exit code of the executed code.
    pub code: u8,
    /// The combined standard output and standard error from the execution.
    pub output: String,
    /// The optional signal that terminated the execution (if applicable).
    pub signal: Option<String>,
    /// The standard output stream from the execution.
    pub stdout: String,
}

impl RunData {
    /// Returns a reference to the standard output stream.
    #[must_use]
    pub fn stdout(&self) -> &str {
        self.stdout.as_ref()
    }

    /// Returns a reference to the combined standard output and standard error.
    #[must_use]
    pub fn output(&self) -> &str {
        self.output.as_ref()
    }

    /// Returns the exit code of the executed code.
    #[must_use]
    pub fn code(&self) -> u8 {
        self.code
    }

    /// Returns an optional reference to the signal that terminated the execution.
    #[must_use]
    pub fn signal(&self) -> Option<&str> {
        self.signal.as_deref()
    }
}
