use email_api::migadu::{error::Error, Mailbox, Migadu};
use rocket::{response::Debug, serde::json::Json, Build, Rocket, State};

use dotenv::dotenv;

#[rocket::get("/domain/<path>")]
async fn domain(path: &str, migadu: &State<Migadu>) -> Result<Json<Vec<Mailbox>>, Debug<Error>> {
    Ok(Json(migadu.mailboxes(path).await?))
}

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let username = std::env::var("MIGADU_API_USERNAME").unwrap();
    let api_key = std::env::var("MIGADU_API_KEY").unwrap();

    let migadu = Migadu::new(username, api_key).unwrap();
    rocket::build()
        .manage(migadu)
        .mount("/api", rocket::routes![domain])
}
