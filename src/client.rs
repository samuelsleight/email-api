use reqwest::{Method, RequestBuilder, Url};
use serde_derive::Deserialize;

use crate::error::Error;

#[derive(Deserialize, Debug)]
pub struct Mailbox {
    #[serde(rename(deserialize = "local_part"))]
    name: String,
}

#[derive(Deserialize)]
struct Mailboxes {
    mailboxes: Vec<Mailbox>,
}

pub(crate) struct Client {
    client: reqwest::Client,
    username: String,
    api_key: String,
}

impl Client {
    pub(crate) fn new(username: String, api_key: String) -> Result<Self, Error> {
        Ok(Self {
            client: reqwest::Client::builder()
                .https_only(true)
                .build()
                .map_err(Error::ClientConstructionError)?,
            username,
            api_key,
        })
    }

    fn request(&self, method: Method, path: &str) -> Result<RequestBuilder, Error> {
        Ok(self
            .client
            .request(
                method,
                Url::parse("https://api.migadu.com/v1/")?.join(path)?,
            )
            .basic_auth(&self.username, Some(&self.api_key)))
    }

    pub(crate) async fn mailboxes(&self, domain: &str) -> Result<Vec<Mailbox>, Error> {
        let response = self
            .request(Method::GET, &format!("domains/{}/mailboxes/", domain))?
            .send()
            .await
            .map_err(Error::MailboxError)?;

        if response.status() != 200 {
            return Err(Error::NoSuchDomain);
        }

        Ok(response
            .json::<Mailboxes>()
            .await
            .map_err(Error::MailboxError)?
            .mailboxes)
    }

    pub(crate) async fn mailbox(&self, domain: &str, mailbox: &str) -> Result<Mailbox, Error> {
        let response = self
            .request(Method::GET, &format!("domains/{}/mailboxes/{}", domain, mailbox))?
            .send()
            .await
            .map_err(Error::MailboxError)?;

        if response.status() != 200 {
            return Err(Error::NoSuchMailbox);
        }

        Ok(response
            .json::<Mailbox>()
            .await
            .map_err(Error::MailboxError)?)
    }
}
