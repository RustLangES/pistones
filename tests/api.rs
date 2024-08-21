#[cfg(test)]
mod client_tests {
    use pistones::client::Client;

    #[tokio::test]
    async fn creating_client() {
        let builder = Client::new()
            .await
            .unwrap()
            .run("rs", "fn main() { println!(\"Hello, world!\") }")
            .await;

        assert!(builder.is_ok());
    }

    #[tokio::test]
    async fn non_passing_main_file() {
        let builder = Client::new().await.unwrap().run("rs", "").await;

        assert!(builder.is_err());
    }

    #[tokio::test]
    async fn non_passing_lang() {
        let builder = Client::new()
            .await
            .unwrap()
            .run("rs", "fn main() { println!(\"Hello, world!\") }")
            .await;

        assert!(builder.is_err());
    }

    #[tokio::test]
    async fn setting_multiple_files() {
        let client = Client::new()
            .await
            .unwrap()
            .run_files(
                "rs",
                [
                    ("utils.rs", "pub fn sum(a: i32, b: i32) -> i32 { a + b }"),
                    (
                        "main.rs",
                        "mod utils;use utils::*;fn main() { println!(\"Result: {}\", sum(5, 6)) }",
                    ),
                ],
            )
            .await;

        assert!(client.is_ok());
    }
}
