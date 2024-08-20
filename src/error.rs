use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown language. See the API instead.")]
    UnknownLang,
    #[error("Missing language.")]
    MissingLang,
    #[error("Missing main / code.")]
    MissingMain,
    #[error("Invalid language.")]
    InvalidLanguage,
    #[error("Bad request formed.")]
    BadRequest,
    #[error("Unkown error catched: {0}.")]
    Unknown(String),
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
}
