use log::*;
use crate::client::Tweet;
use quakes_api::*;
use std::collections::HashMap;
use crate::TwitterError;

const DATETIME_FORMAT: &str = "%d %B %Y - %I:%M %p %#z";

pub(crate) struct TweetParser(Vec<Tweet>);

impl TweetParser {

    fn capture(string: String, prefix: String) -> Option<String> {
        let string_len = string.len();
        let prefix_len = prefix.len();
        if string_len > prefix_len && string.starts_with(&prefix) {
            let stripped = &string[prefix.len()..string.len()];
            Some(stripped.trim().to_string())
        } else {
            None
        }
    }

    fn find_province(text: String) -> (String, String) {
        Quake::find_province(text)
    }

    fn find_latlng(text: String) -> (String, String) {
        let parts: Vec<&str> = text.split(", ").collect();
        if parts.len() == 2 {
            let mut lat = parts[0].to_string();
            lat.truncate(lat.len() - 1);
            let mut lng = parts[1].to_string();
            lng.truncate(lng.len() - 1);
            (lat, lng)
        } else {
            ("".to_string(), "".to_string())
        }
    }

    fn parse_text(text: String) -> HashMap<String, String> {
        let strings: Vec<&str> = text.split("\n").collect();
        let mut map: HashMap<String, String> = HashMap::new();
        for string in strings {
            if let Some(datetime) =  Self::capture(string.to_string(), "Date and Time: ".to_string()) {
                map.insert("datetime".to_string(), format!("{} +08", datetime));
            };
            if let Some(magnitude) = Self::capture(string.to_string(), "Magnitude = ".to_string()) {
                map.insert("magnitude".to_string(), magnitude);
            };
            if let Some(depth) = Self::capture(string.to_string(), "Depth = ".to_string()) {
                if let Some(pos) = depth.rfind(" kilometers") {
                    let stripped = &depth[0..pos];
                    map.insert("depth".to_string(), stripped.to_string());
                }
            };
            if let Some(location) = Self::capture(string.to_string(), "Location = ".to_string()) {
                let parts: Vec<&str> = location.split(" - ").collect();
                if parts.len() == 2 {
                    let (latitude, longitude) = Self::find_latlng(parts[0].to_string());
                    let (location, province) = Self::find_province(parts[1].to_string());
                    map.insert("longitude".to_string(), longitude);
                    map.insert("latitude".to_string(), latitude);
                    map.insert("location".to_string(), location);
                    map.insert("province".to_string(), province);
                }
            };
        }
        map
    }

    fn get_datetime(row: &HashMap<String, String>) -> Result<DateTime<Utc>, TwitterError> {
        let text = row.get("datetime");
        if text.is_some() {
            let value = text.unwrap();
            let datetime: DateTime<FixedOffset> = DateTime::parse_from_str(value, DATETIME_FORMAT)
                .map_err(|error|{
                    TwitterError::new(format!("Trouble converting {} to timestamp: {}", value, error.to_string()))
                })?;
            let utc: DateTime<Utc> = DateTime::from(datetime);
            Ok(utc)
        } else {
            Err(TwitterError::new("Date - Time not found in tweet text!"))
        }
    }

    fn get_longitude(row: &HashMap<String, String>) -> Result<f64, TwitterError> {
        let text = row.get("longitude");
        if text.is_some() {
            let value = text.unwrap();
            value.parse::<f64>().map_err(|error| {
                TwitterError::new(format!("Trouble converting {} to f64: {}", value, error.to_string()))
            })
        } else {
            Err(TwitterError::new("Longitude not found in tweet text!"))
        }
    }

    fn get_latitude(row: &HashMap<String, String>) -> Result<f64, TwitterError> {
        let text = row.get("latitude");
        if text.is_some() {
            let value = text.unwrap();
            value.parse::<f64>().map_err(|error| {
                TwitterError::new(format!("Trouble converting {} to f64: {}", value, error.to_string()))
            })
        } else {
            Err(TwitterError::new("Latitude not found in tweet text!"))
        }
    }

    fn get_magnitude(row: &HashMap<String, String>) -> Result<f64, TwitterError> {
        let text = row.get("magnitude");
        if text.is_some() {
            let value = text.unwrap();
            value.parse::<f64>().map_err(|error| {
                TwitterError::new(format!("Trouble converting {} to f64: {}", value, error.to_string()))
            })
        } else {
            Err(TwitterError::new("Magnitude not found in tweet text!"))
        }
    }

    fn get_depth(row: &HashMap<String, String>) -> Result<u16, TwitterError> {
        let text = row.get("depth");
        if text.is_some() {
            let value = text.unwrap();
            value.parse::<u16>().map_err(|error| {
                TwitterError::new(format!("Trouble converting {} to i8: {}", value, error.to_string()))
            })
        } else {
            Err(TwitterError::new("Depth not found in tweet text!"))
        }
    }

    fn get_location(row: &HashMap<String, String>) -> Result<String, TwitterError> {
        let text = row.get("location");
        if text.is_some() {
            Ok(text.unwrap().clone())
        } else {
            Err(TwitterError::new("Location not found in tweet text!"))
        }
    }

    fn get_province(row: &HashMap<String, String>) -> Result<String, TwitterError> {
        let text = row.get("province");
        if text.is_some() {
            Ok(text.unwrap().clone())
        } else {
            Err(TwitterError::new("province not found in tweet text!"))
        }
    }

    fn get_url(text: Option<String>) -> Result<String, TwitterError> {
        if text.is_some() {
            let url = text.unwrap().replace("http://", "https://");
            Ok(url)
        } else {
            Err(TwitterError::new("URL not found in tweet!"))
        }
    }

    pub(crate) async fn get_quakes(&self) -> Result<Vec<Quake>, TwitterError> {
        let mut quakes: Vec<Quake> = Vec::new();

        for tweet in self.0.clone() {

            if tweet.get_text().contains("Earthquake Information") {
                let row = Self::parse_text(tweet.get_text());

                let datetime = Self::get_datetime(&row)?;
                let longitude = Self::get_longitude(&row)?;
                let latitude = Self::get_latitude(&row)?;
                let magnitude = Self::get_magnitude(&row)?;
                let depth = Self::get_depth(&row)?;
                let location = Self::get_location(&row)?;
                let province = Self::get_province(&row)?;
                let url = Self::get_url(tweet.get_url())?;

                let quake = Quake::new(
                    datetime,
                    longitude,
                    latitude,
                    magnitude,
                    depth,
                    location,
                    province,
                    url
                );
                quakes.push(quake);
            } else {
                debug!("Tweet {} missing #Earthquake tag, skipping:\n{:#?}", &tweet.get_tweet_id(), &tweet);
            }
        }

        Ok(quakes)
    }

    pub fn new(tweets: Vec<Tweet>) -> Self {
        TweetParser(tweets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TWEET_TEXT: &str = "#EarthquakePH #EarthquakeSarangani\nEarthquake Information No.1\nDate and Time: 24 Jan 2020 - 07:21 AM\nMagnitude = 2.3\nDepth = 026 kilometers\nLocation = 06.44N, 125.22E - 019 km N 11° W of Malungon (Sarangani)\n\nhttps://t.co/LzMZu5Gb5t";

    #[test]
    fn parse_tweet_text() {
        let data = TweetParser::parse_text(TWEET_TEXT.to_string());
        println!("{:#?}", data);
    }

    #[actix_rt::test]
    async fn parse_tweets() {

        let tweet = crate::client::tests::get_test_tweet();

        let parser = TweetParser::new(vec![tweet]);
        let quakes = parser.get_quakes().await.unwrap();
        println!("{:#?}", quakes);
    }
}