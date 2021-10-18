use email_api::migadu::Migadu;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let username = std::env::var("MIGADU_API_USERNAME").unwrap();
    let api_key = std::env::var("MIGADU_API_KEY").unwrap();
    let domain = std::env::var("MIGADU_API_TEST_DOMAIN").unwrap();

    let mailbox = std::env::var("MIGADU_API_TEST_NEW_MAILBOX").unwrap();
    let name = std::env::var("MIGADU_API_TEST_NEW_MAILBOX_NAME").unwrap();
    let email = std::env::var("MIGADU_API_TEST_NEW_MAILBOX_EMAIL").unwrap();

    let migadu = Migadu::new(username, api_key).unwrap();
    println!(
        "{:#?}",
        migadu
            .create_mailbox(&domain, &mailbox, &name, &email)
            .await
            .unwrap()
    );
}
