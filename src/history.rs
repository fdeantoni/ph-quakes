use actix_web::client::{Client, Connector};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use std::time::Duration;
use crate::error::QuakeError;

static PHILVOLCS_URL: &str = "https://earthquake.phivolcs.dost.gov.ph/";

async fn ssl_connector() -> Result<SslConnector, std::io::Error> {
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_verify(SslVerifyMode::NONE);
    Ok(builder.build())
}

async fn retrieve_quakes() -> Result<String, QuakeError> {

    let connector = ssl_connector().await?;
    let connector = {
        Connector::new().ssl(connector)
            .timeout(Duration::from_secs(5))
            .finish()
    };

    let mut client = Client::build().connector(connector).finish();

    let mut response = client.get(PHILVOLCS_URL)
        .header("User-Agent", "philvolcs-quakes")
        .send().await?;

    let length = response.headers().get("content-length")
        .map(|header| header.to_str().ok())
        .flatten().map(|string| string.parse::<usize>().ok())
        .flatten().unwrap_or(4_000_000 as usize);

    println!("Length: {}", length);
    let bytes = response.body().limit(length).await?.to_vec();
    let string = std::str::from_utf8(&bytes);

    println!("Response: {:?}", string);
    Ok(String::from("Ok"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_retrieve() {
        retrieve_quakes().await.unwrap();
    }
}