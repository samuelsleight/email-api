use self::{
    client::{Client, Mailbox},
    error::Error
};

mod client;

pub mod error;

pub struct Migadu {
    client: Client,
}

impl Migadu {
    pub fn new(username: String, api_key: String) -> Result<Self, Error> {
        Ok(Self {
            client: Client::new(username, api_key)?,
        })
    }

    pub async fn mailboxes(&self, domain: &str) -> Result<Vec<Mailbox>, Error> {
        Ok(self.client.mailboxes(domain).await?)
    }

    pub async fn mailbox(&self, domain: &str, mailbox: &str) -> Result<Mailbox, Error> {
        Ok(self.client.mailbox(domain, mailbox).await?)
    }

    pub async fn create_mailbox(
        &self,
        domain: &str,
        mailbox: &str,
        name: &str,
        recovery_email: &str,
    ) -> Result<Mailbox, Error> {
        Ok(self
            .client
            .create_mailbox(domain, mailbox, name, recovery_email)
            .await?)
    }
}
