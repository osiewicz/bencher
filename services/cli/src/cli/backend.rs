use std::convert::TryFrom;

use email_address_parser::EmailAddress;
use url::Url;

use crate::cli::clap::CliBackend;
use crate::BencherError;

pub const BENCHER_EMAIL: &str = "BENCHER_EMAIL";
pub const BENCHER_TOKEN: &str = "BENCHER_TOKEN";
pub const BENCHER_URL: &str = "BENCHER_URL";
pub const DEFAULT_URL: &str = "https://api.bencher.dev";

#[derive(Debug)]
pub struct Backend {
    pub email: EmailAddress,
    pub token: String,
    pub url: Url,
}

impl TryFrom<CliBackend> for Backend {
    type Error = BencherError;

    fn try_from(backend: CliBackend) -> Result<Self, Self::Error> {
        Ok(Backend {
            email: map_email(backend.email)?,
            token: map_token(backend.token)?,
            url: map_url(backend.url)?,
        })
    }
}

fn map_email(email: Option<String>) -> Result<EmailAddress, BencherError> {
    if let Some(email) = email {
        map_email_str(email)
    } else if let Ok(email) = std::env::var(BENCHER_EMAIL) {
        map_email_str(email)
    } else {
        Err(BencherError::EmailNotFound)
    }
}

fn map_email_str(email: String) -> Result<EmailAddress, BencherError> {
    EmailAddress::parse(&email, None).ok_or(BencherError::Email(email))
}

fn map_token(token: Option<String>) -> Result<String, BencherError> {
    // TODO add first pass token validation
    if let Some(token) = token {
        Ok(token)
    } else if let Ok(token) = std::env::var(BENCHER_TOKEN) {
        Ok(token)
    } else {
        Err(BencherError::TokenNotFound)
    }
}

fn map_url(url: Option<String>) -> Result<Url, url::ParseError> {
    let url = if let Some(url) = url {
        url
    } else if let Ok(url) = std::env::var(BENCHER_URL) {
        url
    } else {
        DEFAULT_URL.into()
    };
    Ok(Url::parse(&url)?)
}