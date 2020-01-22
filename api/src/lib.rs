use serde_derive::*;
pub use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quake {
    datetime: DateTime<Utc>,
    longitude: f64,
    latitude: f64,
    magnitude: f32,
    location: String,
    url: String
}

impl Quake {
    pub fn get_datetime(&self) -> DateTime<Utc> {
        self.datetime.clone()
    }
    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }
    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }
    pub fn get_magnitude(&self) -> f32 {
        self.magnitude
    }
    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn new(datetime: DateTime<Utc>, longitude: f64, latitude: f64, magnitude: f32, location: String, url: String) -> Quake {
        Quake {
            datetime,
            longitude,
            latitude,
            magnitude,
            location,
            url
        }
    }
}

use std::borrow::Cow;

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