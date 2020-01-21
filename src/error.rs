use std::borrow::Cow;
use std::str::Utf8Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuakeError {
    description: Cow<'static, str>,
}

impl QuakeError {
    pub fn new<S>(description: S) -> Self
        where
            S: Into<Cow<'static, str>>,
    {
        QuakeError {
            description: description.into(),
        }
    }
}

impl std::error::Error for QuakeError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for QuakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Quake error: ")?;
        f.write_str(&self.description)
    }
}

impl From<std::io::Error> for QuakeError {
    fn from(error: std::io::Error) -> Self {
        QuakeError::new(format!(
            "IO error occurred! {}",
            error.to_string()
        ))
    }
}

impl From<actix_web::client::SendRequestError> for QuakeError {
    fn from(error: actix_web::client::SendRequestError) -> Self {
        QuakeError::new(format!(
            "Client error occurred! {}",
            error.to_string()
        ))
    }
}

impl From<actix_web::client::PayloadError> for QuakeError {
    fn from(error: actix_web::client::PayloadError) -> Self {
        QuakeError::new(format!(
            "Client error occurred! {}",
            error.to_string()
        ))
    }
}

impl From<std::str::Utf8Error> for QuakeError {
    fn from(error: Utf8Error) -> Self {
        QuakeError::new(format!(
            "Client error occurred! {}",
            error.to_string()
        ))
    }
}