use serde_derive::*;
pub use chrono::prelude::*;
pub mod time {
    pub use ::chrono::Duration;
}

pub use geojson::{FeatureCollection, Feature, GeoJson, Geometry, Value};
use serde_json::{Map, to_value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quake {
    datetime: DateTime<Utc>,
    longitude: f64,
    latitude: f64,
    magnitude: f64,
    depth: u16,
    location: String,
    province: String,
    url: String
}

fn format_f64(coord: &f64) -> String {
    format!("{:.6}", coord)
}

impl PartialEq for Quake {
    fn eq(&self, other: &Self) -> bool {
        let lng = format_f64(&self.longitude);
        let lat = format_f64(&self.latitude);
        let mag = format_f64(&self.magnitude);
        self.datetime.eq(&other.datetime) &&
            lng.eq(&format_f64(&other.longitude)) &&
            lat.eq(&format_f64(&other.latitude)) &&
            mag.eq(&format_f64(&other.magnitude)) &&
            self.depth.eq(&other.depth)
    }
}

impl Eq for Quake {}

impl Hash for Quake {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let lng = format_f64(&self.longitude);
        let lat = format_f64(&self.latitude);
        let mag = format_f64(&self.magnitude);
        self.datetime.hash(state);
        lng.hash(state);
        lat.hash(state);
        mag.hash(state);
        self.depth.hash(state);
    }
}

impl Ord for Quake {
    fn cmp(&self, other: &Self) -> Ordering {
        self.datetime.cmp(&other.datetime)
    }
}

impl PartialOrd for Quake {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    pub fn get_depth(&self) -> u16 { self.depth }
    pub fn get_location(&self) -> String { self.location.clone() }
    pub fn get_province(&self) -> String { self.province.clone() }
    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn new(datetime: DateTime<Utc>, longitude: f64, latitude: f64, magnitude: f64, depth: u16, location: String, province: String, url: String) -> Quake {
        Quake {
            datetime,
            longitude,
            latitude,
            magnitude,
            depth,
            location,
            province,
            url
        }
    }

    pub fn to_geojson_feature(&self) -> Feature {
        let geometry = Geometry::new(
            Value::Point(vec![self.longitude, self.latitude])
        );
        let mut properties = Map::new();
        properties.insert(
            String::from("datetime"),
            to_value(format!("{:?}", self.datetime)).unwrap(),
        );
        properties.insert(
            String::from("start"),
            to_value(format!("{:?}", self.datetime)).unwrap(),
        );
        properties.insert(
            String::from("end"),
            to_value(format!("{:?}", self.datetime + chrono::Duration::days(1))).unwrap(),
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
            String::from("depth"),
            to_value(self.depth).unwrap(),
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

    pub fn find_province(text: String) -> (String, String) {
        match text.rfind("(") {
            Some(pos) => {
                let len = text.len();
                let province = &text[pos+1..len-1];
                let location = &text[0..pos-1];
                (location.to_string(), province.to_string())
            },
            None => {
                let location = text.clone();
                let mut province = "";
                if let Some(pos) = text.rfind("of ") {
                    province = &text[pos+3..text.len()]
                }
                (location, province.to_string())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct QuakeList(Box<[Quake]>);

impl QuakeList {
    pub fn list(&self) -> Box<[Quake]> {
        self.0.clone()
    }
    pub fn new(vec: Vec<Quake>) -> QuakeList {
        QuakeList(vec.into_boxed_slice())
    }
    pub fn to_geojson(&self) -> GeoJson {
        let bbox = None;
        let foreign_members = None;
        let features: Vec<Feature> = self.0.iter().map(|quake| quake.to_geojson_feature()).collect();
        GeoJson::FeatureCollection(FeatureCollection {
                 bbox,
                 features,
                 foreign_members,
            }
        )
    }
}

use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

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
    use std::collections::HashSet;

    fn test_quake() -> Quake {
        let datetime = Utc::now();
        let longitude: f64 = 1.0;
        let latitude: f64 = 0.0;
        let magnitude: f64 = 2.4;
        let depth: u16 = 134;
        let location = "Some location".to_string();
        let province = "Some province".to_string();
        let url = "http://example.com".to_string();
        Quake::new(datetime, longitude, latitude, magnitude, depth, location, province, url)
    }

    #[test]
    fn geojson_conversion() {
        let quake = test_quake();
        let feature = quake.to_geojson_feature();
        let geojson = GeoJson::Feature(feature);
        println!("{}", geojson.to_string());
    }

    #[test]
    fn compare_quakes() {
        let one = test_quake();
        let mut two = one.clone();
        two.url = "https://some.other.url".to_string();
        assert_eq!(one, two);
    }

    #[test]
    fn compare_quakes_set() {
        let one = test_quake();
        let mut two = one.clone();
        two.url = "https://some.other.url".to_string();
        let mut three = test_quake();
        three.depth = 100;
        let one_vec = vec![one.clone()];
        let two_vec = vec![two.clone(), three.clone()];
        let mut set: HashSet<Quake> = HashSet::new();
        set.extend(one_vec);
        set.extend(two_vec);
        let mut sorted = set.clone().into_iter().collect::<Vec<Quake>>();
        sorted.sort();
        println!("{:?}", &set);
        assert_eq!(sorted[0], one);
        assert_eq!(sorted[1], three);
    }

    #[actix_rt::test]
    async fn retrieve_philvolcs_quakes() {
        let quake = test_quake();
        let list = QuakeList::new(vec![quake]);
        let geojson = list.to_geojson();
        println!("{}", geojson.to_string());
    }
}