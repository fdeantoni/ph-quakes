use awc;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use std::time::Duration;
use crate::ScraperError;

pub struct WebClient(awc::Client);

impl WebClient {

    fn ssl_connector() -> Result<SslConnector, std::io::Error> {
        let mut builder = SslConnector::builder(SslMethod::tls())?;
        builder.set_verify(SslVerifyMode::NONE);
        Ok(builder.build())
    }

    pub async fn retrieve(&self, url: String) -> Result<String, ScraperError> {
        let mut response = self.0.get(url)
            .header("User-Agent", "philvolcs-quakes")
            .send().await?;

        let length = response.headers().get("content-length")
            .map(|header| header.to_str().ok())
            .flatten().map(|string| string.parse::<usize>().ok())
            .flatten().unwrap_or(4_000_000 as usize);

        let bytes = response.body().limit(length).await?.to_vec();
        let string = std::str::from_utf8(&bytes)?;

        Ok(String::from(string))
    }

    pub fn new() -> WebClient {
        let connector = Self::ssl_connector().unwrap();
        let connector = {
            awc::Connector::new().ssl(connector)
                .timeout(Duration::from_secs(5))
                .finish()
        };

        let client = awc::Client::build().connector(connector).finish();
        WebClient(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn retrieve_url() {
        let client = WebClient::new();
        let html = client.retrieve("https://google.com".to_string()).await.unwrap();
        println!("{:?}", html);
        assert!(html.len() > 0);
    }

}



