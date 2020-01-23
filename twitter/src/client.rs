use awc;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use std::time::Duration;
use crate::TwitterError;

pub struct WebClient {
    url: String,
    token: String,
    client: awc::Client
}

impl WebClient {

    fn ssl_connector() -> Result<SslConnector, std::io::Error> {
        let mut builder = SslConnector::builder(SslMethod::tls())?;
        //builder.set_verify(SslVerifyMode::NONE);
        Ok(builder.build())
    }

    async fn get_token(client: &awc::Client, url: String, key: String, secret: String) -> Result<String, TwitterError> {
        let basic = format!("Basic {}:{}", key, secret);
        let uri = format!("{}/oauth2/token", url);
        let mut response = client.post(uri)
            .content_type("application/x-www-form-urlencoded;charset=UTF-8")
            .basic_auth(key, Some(&secret))
            .send_body("grant_type=client_credentials")
            .await?;

        let bytes = response.body().await?.to_vec();
        let string = std::str::from_utf8(&bytes)?;

        Ok(String::from(string))
    }

    //noinspection DuplicatedCode
    pub async fn retrieve(&self, path: String) -> Result<String, TwitterError> {
        let uri = format!("{}/{}", self.url, path);
        let mut response = self.client.get(uri)
            .header("User-Agent", "ph-quakes")
            .header("Authorization", self.token.clone())
            .send().await?;

        let length = response.headers().get("content-length")
            .map(|header| header.to_str().ok())
            .flatten().map(|string| string.parse::<usize>().ok())
            .flatten().unwrap_or(4_000_000 as usize);

        let bytes = response.body().limit(length).await?.to_vec();
        let string = std::str::from_utf8(&bytes)?;

        Ok(String::from(string))
    }

    pub async fn create(url: String, key: String, secret: String) -> Result<WebClient, TwitterError> {
        let connector = Self::ssl_connector().unwrap();
        let connector = {
            awc::Connector::new().ssl(connector)
                .timeout(Duration::from_secs(5))
                .finish()
        };

        let client = awc::Client::build().connector(connector).finish();
        let token = Self::get_token(&client, url.clone(), key, secret).await?;

        Ok(WebClient {
            url,
            token,
            client
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::*;
    use std::env;

    #[actix_rt::test]
    async fn create_client() {
        dotenv().ok();

        std::env::set_var("RUST_LOG", "awc=debug,quakes_server=debug");
        env_logger::init();

        let key = env::var("CONSUMER_KEY").expect("Missing consumer key");
        let secret = env::var("CONSUMER_SECRET").expect("Missing consumer secret");

        let url = "https://api.twitter.com".to_string();
        let client = WebClient::create(url, key, secret).await.unwrap();
        println!("{:?}", &client.token);
    }

    #[actix_rt::test]
    async fn retrieve_timeline() {
        dotenv().ok();

        let key = env::var("CONSUMER_KEY").expect("Missing consumer key");
        let secret = env::var("CONSUMER_SECRET").expect("Missing consumer secret");



    }

}

