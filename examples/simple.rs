use pistones::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().await?;

    let res = client.run("rs", "fn main() { println!(\"Hola\") }").await?;

    println!("Result: {res:?}");

    Ok(())
}
