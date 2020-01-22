pub mod client;
pub mod parser;

use awc;
use std::borrow::Cow;
use std::str::Utf8Error;
use quakes_api::Quake;
use crate::client::WebClient;
use crate::parser::HtmlParser;

static PHILVOLCS_URL: &str = "https://earthquake.phivolcs.dost.gov.ph/";

pub async fn get_philvolcs_quakes() -> Result<Vec<Quake>, ScraperError> {
    let client = WebClient::new();
    let html = client.retrieve(PHILVOLCS_URL.to_string()).await?;
    let parser = HtmlParser::parse(html, PHILVOLCS_URL.to_string()).await;
    parser.get_quakes().await
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScraperError {
    description: Cow<'static, str>,
}

impl ScraperError {
    pub fn new<S>(description: S) -> Self
        where
            S: Into<Cow<'static, str>>,
    {
        ScraperError {
            description: description.into(),
        }
    }
}

impl std::error::Error for ScraperError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for ScraperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Scraper error: ")?;
        f.write_str(&self.description)
    }
}

impl From<std::io::Error> for ScraperError {
    fn from(error: std::io::Error) -> Self {
        ScraperError::new(format!(
            "IO error in Scraper: {}",
            error.to_string()
        ))
    }
}

impl From<awc::error::PayloadError> for ScraperError {
    fn from(error: awc::error::PayloadError) -> Self {
        ScraperError::new(format!(
            "Scraper payload error: {}",
            error.to_string()
        ))
    }
}

impl From<awc::error::SendRequestError> for ScraperError {
    fn from(error: awc::error::SendRequestError) -> Self {
        ScraperError::new(format!(
            "Scraper send error: {}",
            error.to_string()
        ))
    }
}

impl From<Utf8Error> for ScraperError {
    fn from(error: Utf8Error) -> Self {
        ScraperError::new(format!(
            "Scraper decoding error: {}",
            error.to_string()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test] #[ignore]
    async fn retrieve_philvolcs_quakes() {
        let quakes = get_philvolcs_quakes().await.unwrap();
        println!("{:?}", quakes);
        assert!(quakes.len() > 0);
    }

}