use pistones::{lang::Response, Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = compile_code("rust", "fn main() { println!(\"Hola\") }").await?;
    println("{response:?}");
    Ok(())
}

async fn compile_code(lang: &str, code: &str) -> Result<Response, Error> {
    let client = Client::new().await?.user_agent("@romancitodev")?;
    client.run(lang, code).await
}
