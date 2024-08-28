# 🚂 pistones

Unofficial API Client wrapper from [engineer-man/piston](https://github.com/engineer-man/piston?tab=readme-ov-file#Public-API)

### Functionality

The Piston client provides functionalities to:

* **Create a Piston client:** Establish a connection with the Piston execution environment.
* **Configure the client:**
    * Set the API version.
    * Set the base URL for the Piston API.
    * Enable or disable language caching.
    * Define a custom `reqwest` client for HTTP requests.
* **Language Management:**
    * Fetch a list of supported languages and their versions.
    * Get the version for a specific language.
* **Code Execution:**
    * Execute Piston code provided as strings or files.
    * Specify the language and version for code execution.

### Usage

The library utilizes an asynchronous programming model. Here's a basic example demonstrating how to run a Piston script:

```rust
use piston_client::Client;

#[tokio::main]
fn main() -> Result<(), piston_client::Error>> {
    let mut client = Client::new()?;

    // Get the Rust language version
    let rust_version = client.lang_version("rust").await?;
    println!("Rust version: {}", rust_version);

    // Define the code to run
    let code = r#"
        fn main() {
            println!("Hello, Piston!");
        }
    "#;

    // Run the code
    let response = client.run("rust", code.to_string()).await?;

    // Process the response
    println!("Response: {:?}", response);

    Ok(())
}
```

### Client Configuration

* **API Version:** By default, the client uses the default API version. You can change this using the `api_version` method.
* **Base URL:** The base URL for the Piston API defaults to `https://emkc.org`. You can override this with the `base_url` method.
* **Language Caching:** Language information is fetched from the Piston API by default and cached for subsequent use. You can disable caching with the `disable_cache` method.
* **Custom Client:** The library utilizes `reqwest` for making HTTP requests. You can provide your own `reqwest::Client` instance using the `custom_client` method.

### Client Methods

* **new:** Creates a new Piston client instance.
* **api_version:** Sets the API version to be used by the client.
* **base_url:** Sets the base URL for the Piston API.
* **disable_cache:** Disables language information caching.
* **user_agent:** Sets a custom user agent for HTTP requests.
* **custom_client:** Sets a custom `reqwest` client for HTTP requests.
* **refresh_cache:** Updates the cached language information.
* **get_languages:** Retrieves a list of supported languages and their versions.
* **lang_version:** Gets the version for a specific language.
* **exec:** Executes Piston code provided as files.
* **run_files:** Executes Piston code provided through an iterator of `FileData` structs.
* **run_with_version:** Executes Piston code with a specified language version and content string.
* **run:** Executes Piston code with a retrieved language version and content string.

### Data Structures

* **Language:** Represents a supported language with its name, aliases, and version.
* **FileData:** Represents a file to be included in the code execution. It has optional `name` and required `content` fields.
* **Error:** Represents errors that can occur during library usage.
* **ApiResponse:** Represents the response received from the Piston API after code execution. It can be either a successful execution result or an error message.

### Contributing

Feel free to contribute to this library by opening pull requests on the relevant Github repository. Make sure to follow the project's contribution guidelines.
