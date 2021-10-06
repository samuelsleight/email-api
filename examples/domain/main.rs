use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let username = std::env::var("MIGADU_API_USERNAME").unwrap();
    let api_key = std::env::var("MIGADU_API_KEY").unwrap();
    let domain = std::env::var("MIGADU_API_TEST_DOMAIN").unwrap();

    let mut migadu = migadu::Migadu::new(username, api_key).unwrap();
    println!("{:#?}", migadu.domain(&domain).await.unwrap());
}
