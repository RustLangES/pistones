use pistones::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().await?;

    let res = client.get_languages().await?;

    println!("Result: {res:?}");

    Ok(())
}
