pub mod client;
pub mod parser;

use awc;
use std::borrow::Cow;
use std::str::Utf8Error;

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