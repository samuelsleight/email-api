use dotenv::dotenv;
use email_api::migadu::Migadu;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let username = std::env::var("MIGADU_API_USERNAME").unwrap();
    let api_key = std::env::var("MIGADU_API_KEY").unwrap();
    let domain = std::env::var("MIGADU_API_TEST_DOMAIN").unwrap();

    let migadu = Migadu::new(username, api_key).unwrap();
    println!("{:#?}", migadu.mailboxes(&domain).await.unwrap());
}
