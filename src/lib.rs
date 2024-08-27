//! **Piston Client Library**
//!
//! This library provides an interface for interacting with the Piston execution platform.
//! It allows you to send code written in various programming languages to Piston for execution and receive the results.
//!
//! ## Key Features
//!
//! * **Code execution:** Execute code in various programming languages supported by Piston.
//! * **Version management:** Specify the language version to use.
//! * **File handling:** Send multiple files as part of the code to be executed.
//! * **Language caching:** Optionally cache language information for performance improvements.
//! * **Customization:** Configure the base URL of the API, user agent, and use a custom HTTP client.
//!
//! ## Basic Usage
//!
//! ```rust
//! use piston_client::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), piston_client::Error> {
//!     let mut client = Client::new().await?;
//!
//!     // Execute Rust code
//!     let code = r#"
//!         fn main() {
//!             println!("Hello from Piston!");
//!         }
//!     "#;
//!     let response = client.run("rust", code.to_string()).await?;
//!
//!     println!("Response: {:?}", response);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Structure
//!
//! The library primarily consists of:
//!
//! * **`Client`:** The main structure representing a Piston client.
//! * **`Data`:** Represents the data to be sent for code execution.
//! * **`FileData`:** Represents a file to be included in the execution.
//! * **`ApiVersion`:** Enumeration representing different versions of the Piston API.
//!
//! ## More Information
//!
* **Detailed documentation:** For more details on the functionalities and usage of the library, refer to the detailed documentation of each structure and method.
//!
//! **Note:** This is a basic documentation example. You can customize and expand it to include more details and examples as needed.
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
