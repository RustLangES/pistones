use pistones::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().await?.user_agent("@romancitodev")?;

    tokio::spawn(async move {
        let _ = client
            .run("rust", "fn main() { println!(\"Hola\") }")
            .await
            .unwrap();
    });

    println!("hello!");
    Ok(())
}
