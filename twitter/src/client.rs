use awc;
use openssl::ssl::{SslConnector, SslMethod};
use std::time::Duration;
use serde_derive::*;
use crate::TwitterError;

#[derive(Clone)]
pub struct TwitterClient {
    url: String,
    client: awc::Client,
    key: String,
    secret: String,
    token: String,
}

#[derive(Debug,Deserialize)]
struct TokenResponse {
    token_type: String,
    access_token: String
}

#[derive(Debug,Clone,Deserialize)]
pub struct Url {
    expanded_url: String
}

#[derive(Debug,Clone,Deserialize)]
pub struct Entities {
    urls: Vec<Url>,
}

#[derive(Debug,Clone,Deserialize)]
pub struct Tweet {
    created_at: String,
    id: u64,
    full_text: String,
    entities: Entities,
}

impl Tweet {
    pub fn get_text(&self) -> String {
        self.full_text.clone()
    }
    pub fn get_url(&self) -> Option<String> {
        self.entities.urls.last().into_iter().map(|url| url.expanded_url.clone()).last()
    }
}

impl TwitterClient {

    fn ssl_connector() -> Result<SslConnector, std::io::Error> {
        let builder = SslConnector::builder(SslMethod::tls())?;
        Ok(builder.build())
    }

    async fn get_token(&self) -> Result<String, TwitterError> {
        let uri = format!("{}/oauth2/token", &self.url);
        let mut response = self.client.post(uri)
            .content_type("application/x-www-form-urlencoded;charset=UTF-8")
            .basic_auth(self.key.clone(), Some(&self.secret))
            .send_body("grant_type=client_credentials")
            .await?;

        let bytes = response.body().await?.to_vec();
        let string = std::str::from_utf8(&bytes)?;

        let parsed: TokenResponse = serde_json::from_str(string).unwrap();

        Ok(String::from(parsed.access_token))
    }

    pub async fn timeline(&mut self, screen_name: String, last_tweet_id: Option<u64>) -> Result<Vec<Tweet>, TwitterError> {

        if self.token.is_empty() {
            self.token = self.get_token().await?
        }

        let path = "/1.1/statuses/user_timeline.json";
        let parameters = format!("tweet_mode=extended&screen_name={}", screen_name);

        let uri = if last_tweet_id.is_none() {
            format!("{}/{}?{}&count={}", self.url, path, parameters, 5)
        } else {
            format!("{}/{}?{}&since_id={}", self.url, path, parameters, last_tweet_id.unwrap())
        };

        let mut response = self.client.get(uri)
            .header("User-Agent", "ph-quakes")
            .bearer_auth(self.token.clone())
            .send().await?;

        let bytes = response.body().limit(10_000_000).await?.to_vec();
        let string = std::str::from_utf8(&bytes)?;

        let tweets: Vec<Tweet> = serde_json::from_str(string).unwrap();

        Ok(tweets)
    }

    pub fn new(url: String, key: String, secret: String) -> TwitterClient {
        let connector = Self::ssl_connector().unwrap();
        let connector = {
            awc::Connector::new().ssl(connector)
                .timeout(Duration::from_secs(5))
                .finish()
        };

        let client = awc::Client::build().connector(connector).finish();
        let token= "".to_string();

        TwitterClient {
            url,
            client,
            key,
            secret,
            token,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use dotenv::*;
    use std::env;

    use std::sync::Once;

    static INIT: Once = Once::new();

    static mut CLIENT: Option<TwitterClient> = None;

    fn init() -> TwitterClient {
        unsafe {
            INIT.call_once(|| {
                dotenv().ok();

                let key = env::var("CONSUMER_KEY").expect("Missing consumer key");
                let secret = env::var("CONSUMER_SECRET").expect("Missing consumer secret");

                let url = "https://api.twitter.com".to_string();
                let client = TwitterClient::new(url, key, secret);
                CLIENT = Some(client);
            });

            CLIENT.clone().unwrap()
        }
    }

    #[actix_rt::test] #[ignore]
    async fn create_client() {

        let client = init();
        let token = client.get_token().await.unwrap();
        println!("{:?}", token);
    }

    #[actix_rt::test] #[ignore]
    async fn retrieve_timeline() {

        let mut client = init();
        let screen_name = "phivolcs_dost".to_string();
        let tweets = client.timeline(screen_name, None).await.unwrap();
        println!("{:#?}", tweets);
        assert!(tweets.len() > 0);
        println!("Text: {:?}", tweets[0].get_text());
        println!("Url: {:?}", tweets[0].get_url());
    }

    const TWEET_TEXT: &str = r#"#EarthquakePH #EarthquakeSarangani\nEarthquake Information No.1\nDate and Time: 24 Jan 2020 - 07:21 AM\nMagnitude = 2.3\nDepth = 026 kilometers\nLocation = 06.44N, 125.22E - 019 km N 11° W of Malungon (Sarangani)\n\nhttps://t.co/LzMZu5Gb5t"#;

    pub(crate) fn get_test_tweet() -> Tweet {
        Tweet {
            created_at: "".to_string(),
            id: 0,
            full_text: TWEET_TEXT.to_string(),
            entities: Entities {
                urls: vec![ Url {
                    expanded_url: "http://example.com".to_string()
                }]
            }
        }
    }
}

