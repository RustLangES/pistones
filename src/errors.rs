use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
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
}
