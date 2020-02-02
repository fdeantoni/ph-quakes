pub mod client;
pub mod parser;

use awc;
use std::borrow::Cow;
use std::str::Utf8Error;

use quakes_api::*;
use crate::client::WebClient;
use crate::parser::HtmlParser;
use std::collections::HashSet;

static PHIVOLCS_URL: &str = "https://earthquake.phivolcs.dost.gov.ph/";

async fn retrieve_previous_month(client: &WebClient) -> Result<Vec<Quake>, ScraperError> {
    let current = Utc::today();
    let horizon = Utc::today() - time::Duration::weeks(4);
    if horizon.month() < current.month() {
        let url = format!("{}{}.html", PHIVOLCS_URL, horizon.format("%Y_%B"));
        let html = client.retrieve(url).await?;
        let parser = HtmlParser::parse(html, PHIVOLCS_URL.to_string()).await;
        parser.get_quakes().await
    } else {
        Ok(Vec::new())
    }
}

async fn retrieve_current_month(client: &WebClient) -> Result<Vec<Quake>, ScraperError> {
    let html = client.retrieve(PHIVOLCS_URL.to_string()).await?;
    let parser = HtmlParser::parse(html, PHIVOLCS_URL.to_string()).await;
    parser.get_quakes().await
}

pub async fn get_phivolcs_quakes() -> Result<Vec<Quake>, ScraperError> {
    let client = WebClient::new();
    let mut set: HashSet<Quake> = HashSet::new();
    let current = retrieve_current_month(&client).await?;
    set.extend(current);
    let history = retrieve_previous_month(&client).await?;
    set.extend(history);
    let mut quakes: Vec<Quake> = set.into_iter().collect();
    quakes.sort();
    Ok(quakes)
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
    async fn retrieve_phivolcs_quakes() {
        let quakes = get_phivolcs_quakes().await.unwrap();
        println!("{:?}", &quakes);
        println!("{}", &quakes.len());
        assert!(quakes.len() > 0);
    }
}