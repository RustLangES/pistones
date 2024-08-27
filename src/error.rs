//! **Piston Client Library Error Types**
//!
//! This module defines error types that can occur while using the Piston client library.
use thiserror::Error;

/// Represents various errors that can occur during interactions with the Piston API.
#[derive(Error, Debug)]
pub enum Error {
    /// Indicates that the specified language is unknown or not supported.
    #[error("Unknown language. See the API instead.")]
    UnknownLang,

    /// Indicates that a language was not provided.
    #[error("Missing language.")]
    MissingLang,

    /// Indicates that the main or code file was not provided.
    #[error("Missing main / code.")]
    MissingMain,

    /// Indicates that the specified language is invalid.
    #[error("Invalid language.")]
    InvalidLanguage,

    /// Indicates that a malformed request was sent to the Piston API.
    #[error("Bad request formed.")]
    BadRequest,

    /// Indicates an unknown error occurred during the request.
    #[error("Unknown error caught: {0}.")]
    Unknown(String),

    /// Indicates an error occurred during the HTTP request.
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
}
