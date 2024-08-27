pub mod client;
pub mod error;
pub mod lang;

extern crate reqwest;

/// Use only for GET method
pub const RUNTIMES_PATH: &str = "/piston/runtimes";
/// Use only for POST method
pub const EXECUTE_PATH: &str = "/piston/execute";

pub use client::Client;
pub use error::Error;
