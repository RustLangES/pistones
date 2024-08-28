use pistones::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Normal client
    let _ = Client::new().await?;
    // Create client with cache disabled
    let client = Client::new().await?.disable_cache();

    // Run code refreshing langs cache
    let res = client
        .refresh_cache()
        .await?
        .run("rust", "fn main() { println!(\"Hola\") }")
        .await?;

    println!("Result: {res:?}");

    Ok(())
}
