use crate::{
    client::{Client, Mailbox},
    error::Error,
};

mod client;
mod error;

pub struct Migadu {
    client: Client,
}

#[derive(Debug)]
pub struct Domain {
    domain: String,
    mailboxes: Vec<Mailbox>,
}

impl Migadu {
    pub fn new(username: String, api_key: String) -> Result<Self, Error> {
        Ok(Self {
            client: Client::new(username, api_key)?,
        })
    }

    pub async fn domain(&mut self, domain: &str) -> Result<Domain, Error> {
        let mailboxes = self.client.mailboxes(domain).await?;

        Ok(Domain {
            domain: domain.to_string(),
            mailboxes,
        })
    }
}
