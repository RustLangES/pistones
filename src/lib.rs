pub mod client;
pub mod consts;
pub mod errors;
pub mod lang;

#[cfg(test)]
mod client_tests {
    use crate::client::ClientBuilder;

    #[tokio::test]
    async fn creating_client() {
        let builder = ClientBuilder::new()
            .set_lang("rs")
            .set_main_file("fn main() { println!(\"Hello, world!\") }")
            .build()
            .map_err(|err| println!("{:?}", err));

        assert!(matches!(builder, Ok(_)));
    }

    #[tokio::test]
    async fn non_passing_main_file() {
        let builder = ClientBuilder::new().set_lang("rs").build();

        assert!(matches!(builder, Err(_)));
    }

    #[tokio::test]
    async fn non_passing_lang() {
        let builder = ClientBuilder::new()
            .set_main_file("fn main() { println!(\"Hello, world!\") }")
            .build();

        assert!(matches!(builder, Err(_)));
    }

    #[tokio::test]
    async fn sending_post() {
        let client = ClientBuilder::new()
            .set_lang("rust")
            .set_main_file("fn main() { println!(\"Hello, world!\") }")
            .build()
            .unwrap();

        let result = client.execute().await.map_err(|err| println!("{:?}", err));

        assert!(matches!(result, Ok(_)));
    }

    #[tokio::test]
    async fn setting_multiple_files() {
        let client = ClientBuilder::new()
            .set_lang("rs")
            .set_main_file("fn main() { println!(\"Hello, World!\") }")
            // .add_files(vec!["pub mod add(a: i32, b: i32) -> i32 { a + b }"])
            .build()
            .unwrap();
        let response = client.execute().await.unwrap();
        let data = response.data();
        let output = data.output();
        let signal = data.signal();
        let code = response.data().code();
        println!("output: {output} - {signal:?} - {code}");
    }
}
