use serde_derive::*;
pub use chrono::prelude::*;
pub use geojson::{FeatureCollection, Feature, GeoJson, Geometry, Value};
use serde_json::{Map, to_value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quake {
    datetime: DateTime<Utc>,
    longitude: f64,
    latitude: f64,
    magnitude: f64,
    location: String,
    province: String,
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
    pub fn get_magnitude(&self) -> f64 {
        self.magnitude
    }
    pub fn get_location(&self) -> String { self.location.clone() }
    pub fn get_province(&self) -> String { self.province.clone() }
    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn new(datetime: DateTime<Utc>, longitude: f64, latitude: f64, magnitude: f64, location: String, province: String, url: String) -> Quake {
        Quake {
            datetime,
            longitude,
            latitude,
            magnitude,
            location,
            province,
            url
        }
    }

    pub fn to_geojso_featuren(&self) -> Feature {
        let geometry = Geometry::new(
            Value::Point(vec![self.longitude, self.latitude])
        );
        let mut properties = Map::new();
        properties.insert(
            String::from("datetime"),
            to_value(format!("{:?}", self.datetime)).unwrap(),
        );
        properties.insert(
            String::from("longitude"),
            to_value(self.longitude).unwrap(),
        );
        properties.insert(
            String::from("latitude"),
            to_value(self.latitude).unwrap(),
        );
        properties.insert(
            String::from("magnitude"),
            to_value(self.magnitude).unwrap(),
        );
        properties.insert(
            String::from("location"),
            to_value(self.location.clone()).unwrap(),
        );
        properties.insert(
            String::from("province"),
            to_value(self.province.clone()).unwrap(),
        );
        properties.insert(
            String::from("url"),
            to_value(self.url.clone()).unwrap(),
        );

        Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
            properties: Some(properties),
            foreign_members: None,
        }
    }
}

pub struct QuakeList(Box<[Quake]>);

impl QuakeList {
    pub fn list(&self) -> Box<[Quake]> {
        self.0.clone()
    }
    pub fn new(vec: Vec<Quake>) -> QuakeList {
        QuakeList(vec.into_boxed_slice())
    }
    pub async fn to_geojson(&self) -> FeatureCollection {
        let bbox = None;
        let foreign_members = None;
        let features: Vec<Feature> = self.0.iter().map(|quake| quake.to_geojso_featuren()).collect();
        FeatureCollection {
                 bbox,
                 features,
                 foreign_members,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_quake() -> Quake {
        let datetime = Utc::now();
        let longitude: f64 = 1.0;
        let latitude: f64 = 0.0;
        let magnitude: f64 = 2.4;
        let location = "Some location".to_string();
        let province = "Some province".to_string();
        let url = "http://example.com".to_string();
        Quake::new(datetime, longitude, latitude, magnitude, location, province, url)
    }

    #[test]
    fn geojson_conversion() {
        let quake = test_quake();
        let feature = quake.to_geojso_featuren();
        let geojson = GeoJson::Feature(feature);
        println!("{}", geojson.to_string());
    }

    #[actix_rt::test]
    async fn retrieve_philvolcs_quakes() {
        let quake = test_quake();
        let list = QuakeList::new(vec![quake]);
        let geojson = list.to_geojson().await;
        println!("{}", geojson.to_string());
    }
}