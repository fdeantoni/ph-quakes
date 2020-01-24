pub mod client;
pub mod parser;

use awc;
use std::borrow::Cow;
use std::str::Utf8Error;
use quakes_api::*;
use crate::client::{TwitterClient, Tweet};
use crate::parser::TweetParser;

const PHIVOLCS_SCREEN_NAME: &str = "phivolcs_dost";
const TWITTER_URL: &str = "https://api.twitter.com";

pub struct TwitterQuakes {
    client: TwitterClient,
    last_tweet_id: u64
}

impl TwitterQuakes {

    async fn process(&mut self, tweets: Vec<Tweet>) -> Result<Vec<Quake>, TwitterError> {
        if !tweets.is_empty() {
            self.last_tweet_id = tweets.last().unwrap().get_tweet_id();
        }
        let parser = TweetParser::new(tweets);
        parser.get_quakes().await
    }

    fn has_started(&self) -> bool {
        self.last_tweet_id > 0
    }

    async fn start(&mut self) -> Result<Vec<Quake>, TwitterError> {
        let screen_name = PHIVOLCS_SCREEN_NAME.to_string();
        let last_tweet_id = None;
        let tweets = self.client.timeline(screen_name, last_tweet_id).await?;
        self.process(tweets).await
    }

    async fn next(&mut self) -> Result<Vec<Quake>, TwitterError> {
        let screen_name = PHIVOLCS_SCREEN_NAME.to_string();
        let last_tweet_id = Some(self.last_tweet_id);
        let tweets = self.client.timeline(screen_name, last_tweet_id).await?;
        self.process(tweets).await
    }

    pub async fn get_tweets(&mut self) -> Result<Vec<Quake>, TwitterError> {
        if !self.has_started() {
            self.start().await
        } else {
            self.next().await
        }
    }

    pub fn new(key: String, secret: String) -> Self {
        let url = TWITTER_URL.to_string();
        let client = TwitterClient::new(url, key, secret);
        let last_tweet_id: u64 = 0;
        TwitterQuakes {
            client,
            last_tweet_id
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TwitterError {
    description: Cow<'static, str>,
}

impl TwitterError {
    pub fn new<S>(description: S) -> Self
        where
            S: Into<Cow<'static, str>>,
    {
        TwitterError {
            description: description.into(),
        }
    }
}

impl std::error::Error for TwitterError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for TwitterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Twitter error: ")?;
        f.write_str(&self.description)
    }
}

impl From<std::io::Error> for TwitterError {
    fn from(error: std::io::Error) -> Self {
        TwitterError::new(format!(
            "IO error in Twitter: {}",
            error.to_string()
        ))
    }
}

impl From<awc::error::PayloadError> for TwitterError {
    fn from(error: awc::error::PayloadError) -> Self {
        TwitterError::new(format!(
            "Twitter payload error: {}",
            error.to_string()
        ))
    }
}

impl From<awc::error::SendRequestError> for TwitterError {
    fn from(error: awc::error::SendRequestError) -> Self {
        TwitterError::new(format!(
            "Twitter send error: {}",
            error.to_string()
        ))
    }
}

impl From<Utf8Error> for TwitterError {
    fn from(error: Utf8Error) -> Self {
        TwitterError::new(format!(
            "Scraper decoding error: {}",
            error.to_string()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::*;
    use std::env;

    #[actix_rt::test] #[ignore]
    async fn start_twitter_quakes() {
        dotenv().ok();
        let key = env::var("CONSUMER_KEY").expect("Missing consumer key");
        let secret = env::var("CONSUMER_SECRET").expect("Missing consumer secret");

        let mut twitter = TwitterQuakes::new(key, secret);
        let quakes = twitter.start().await.unwrap();
        println!("{:#?}", quakes);
    }

}