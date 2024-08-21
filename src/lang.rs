use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Language {
    pub language: String,
    pub version: String,
    pub aliases: Vec<String>,
    pub runtime: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Good(Response),
    Error(ErrorResponse),
}

#[derive(Debug, Deserialize)]
pub struct Response {
    language: String,
    version: String,
    run: RunData,
}

impl Response {
    #[must_use]
    pub fn language(&self) -> &str {
        self.language.as_ref()
    }

    #[must_use]
    pub fn version(&self) -> &str {
        self.version.as_ref()
    }

    #[must_use]
    pub fn data(&self) -> &RunData {
        &self.run
    }
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    message: String,
}

impl ErrorResponse {
    #[must_use]
    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}

#[derive(Debug, Deserialize)]
pub struct RunData {
    code: u8,
    output: String,
    signal: Option<String>,
    stdout: String,
}

impl RunData {
    #[must_use]
    pub fn stdout(&self) -> &str {
        self.stdout.as_ref()
    }

    #[must_use]
    pub fn output(&self) -> &str {
        self.output.as_ref()
    }

    #[must_use]
    pub fn code(&self) -> u8 {
        self.code
    }

    #[must_use]
    pub fn signal(&self) -> Option<&str> {
        self.signal.as_deref()
    }
}
