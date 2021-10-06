use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error initialising client")]
    ClientConstructionError(reqwest::Error),

    #[error("Error parsing URL string")]
    UrlParseError(#[from] url::ParseError),

    #[error("Domain does not exist")]
    NoSuchDomain,

    #[error("Error getting mailboxes")]
    MailboxError(reqwest::Error),
}
