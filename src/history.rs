use actix_web::client::{Client, Connector};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use std::time::Duration;
use crate::error::QuakeError;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use scraper::element_ref::Text;
use chrono::prelude::*;

static PHILVOLCS_URL: &str = "https://earthquake.phivolcs.dost.gov.ph/";

async fn ssl_connector() -> Result<SslConnector, std::io::Error> {
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_verify(SslVerifyMode::NONE);
    Ok(builder.build())
}

async fn retrieve_html() -> Result<String, QuakeError> {

    let connector = ssl_connector().await?;
    let connector = {
        Connector::new().ssl(connector)
            .timeout(Duration::from_secs(5))
            .finish()
    };

    let client = Client::build().connector(connector).finish();

    let mut response = client.get(PHILVOLCS_URL)
        .header("User-Agent", "philvolcs-quakes")
        .send().await?;

    let length = response.headers().get("content-length")
        .map(|header| header.to_str().ok())
        .flatten().map(|string| string.parse::<usize>().ok())
        .flatten().unwrap_or(4_000_000 as usize);

    let bytes = response.body().limit(length).await?.to_vec();
    let string = std::str::from_utf8(&bytes)?;

    Ok(String::from(string))
}

fn sanitize_text(text: Text<'_>) -> Vec<&str> {
    text
        .into_iter().map(|string| string.trim() )
        .filter(|string| !string.is_empty() )
        .collect()
}

async fn parse_html(html: String) -> Vec<HashMap<String, String>> {
    let mut collection = Vec::new();
    let expected_headers: HashSet<String> = [
            String::from("Date - Time"),
            String::from("Latitude"),
            String::from("Longitude"),
            String::from("Depth"),
            String::from("Mag"),
            String::from("Location")
        ].iter().cloned().collect();

    let document = Html::parse_document(&html);
    let table_selector = Selector::parse("table").unwrap();
    let th_selector = Selector::parse("th p").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let url_selector = Selector::parse("a").unwrap();

    document.select(&table_selector).into_iter().for_each(|table| {
        let mut table_headers= Vec::new();
        for th in table.select(&th_selector) {
            let mut list = th.text().collect::<Vec<_>>();
            list.reverse();
            list.pop().into_iter().for_each(|header| {
                table_headers.push(header.trim().to_string());
            });
        }
        if HashSet::from_iter(table_headers.iter().cloned()).eq(&expected_headers) {
            for tr in table.select(&tr_selector) {
                let mut row = HashMap::new();
                for (idx, td) in tr.select(&td_selector).enumerate() {
                    let idx_header = idx.to_string();
                    let header = table_headers.get(idx).unwrap_or(&idx_header).clone();
                    if header.eq("Date - Time") {
                        let url = td.select(&url_selector).last().unwrap().value().attr("href").unwrap_or("");
                        let url = url.replace("\\", "/");
                        row.insert("url".to_string(), format!("{}{}", PHILVOLCS_URL, url).to_string());
                        let text = sanitize_text(td.text())
                            .last().cloned().unwrap_or("error");
                        row.insert("Date - Time".to_string(), format!("{} +08", text));
                    } else if header.eq("Location") {
                        let text: String = sanitize_text(td.text()).join("");
                        row.insert("Location".to_string(), text);
                    } else {
                        row.insert(header, td.inner_html().trim().to_string());
                    }
                }
                if !row.is_empty() {
                    collection.push(row);
                }
            };
        }
    });
    collection
}

#[derive(Debug, Clone, PartialEq)]
pub struct Quake {
    datetime: DateTime<Utc>,
    longitude: f64,
    latitude: f64,
    magnitude: f32,
    location: String,
    url: String
}

const DATETIME_FORMAT: &str = "%d %B %Y - %H:%M %p %#z";

fn get_datetime(row: &HashMap<String, String>) -> Result<DateTime<Utc>, QuakeError> {
    let text = row.get("Date - Time");
    if text.is_some() {
        let datetime: DateTime<FixedOffset> = DateTime::parse_from_str(text.unwrap(), DATETIME_FORMAT)
            .map_err(|error|{
                QuakeError::new(format!("Trouble converting datetime string to timestamp: {}", error.to_string()))
            })?;
        let utc: DateTime<Utc> = DateTime::from(datetime);
        Ok(utc)
    } else {
        Err(QuakeError::new("Date - Time not found in row!"))
    }
}

fn get_longitude(row: &HashMap<String, String>) -> Result<f64, QuakeError> {
    let text = row.get("Longitude");
    if text.is_some() {
        let value = text.unwrap();
        value.parse::<f64>().map_err(|error| {
            QuakeError::new(format!("Trouble converting {} to f64: {}", value, error.to_string()))
        })
    } else {
        Err(QuakeError::new("Longitude not found in row!"))
    }
}

fn get_latitude(row: &HashMap<String, String>) -> Result<f64, QuakeError> {
    let text = row.get("Latitude");
    if text.is_some() {
        let value = text.unwrap();
        value.parse::<f64>().map_err(|error| {
            QuakeError::new(format!("Trouble converting {} to f64: {}", value, error.to_string()))
        })
    } else {
        Err(QuakeError::new("Latitude not found in row!"))
    }
}

fn get_magnitude(row: &HashMap<String, String>) -> Result<f32, QuakeError> {
    let text = row.get("Mag");
    if text.is_some() {
        let value = text.unwrap();
        value.parse::<f32>().map_err(|error| {
            QuakeError::new(format!("Trouble converting {} to f32: {}", value, error.to_string()))
        })
    } else {
        Err(QuakeError::new("Mag not found in row!"))
    }
}

fn get_location(row: &HashMap<String, String>) -> Result<String, QuakeError> {
    let text = row.get("Location");
    if text.is_some() {
        Ok(text.unwrap().clone())
    } else {
        Err(QuakeError::new("Location not found in row!"))
    }
}

fn get_url(row: &HashMap<String, String>) -> Result<String, QuakeError> {
    let text = row.get("url");
    if text.is_some() {
        Ok(text.unwrap().clone())
    } else {
        Err(QuakeError::new("URL not found in row!"))
    }
}

async fn parse_quakes(table: Vec<HashMap<String, String>>) -> Result<Vec<Quake>, QuakeError> {
    let mut quakes = Vec::new();

    for row in table {

        let datetime = get_datetime(&row)?;
        let longitude = get_longitude(&row)?;
        let latitude = get_latitude(&row)?;
        let magnitude = get_magnitude(&row)?;
        let location = get_location(&row)?;
        let url = get_url(&row)?;

        let quake = Quake {
            datetime,
            longitude,
            latitude,
            magnitude,
            location,
            url
        };
        quakes.push(quake);
    }
    Ok(quakes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn retrieve_html_test() {
        let html = retrieve_html().await.unwrap();
        println!("{:?}", html);
        assert!(html.len() > 0);
    }

    const HTML: &str = r#"
            <table style="width:100%">
              <tr>
                <th><p>Firstname</p></th>
                <th><p>Lastname</p></th>
                <th><p>Age</p></th>
              </tr>
              <tr>
                <td>Jill</td>
                <td>Smith</td>
                <td>50</td>
              </tr>
            </table>
            <table style="width:100%">
              <tr>
                <th><p>Date - Time</p></th>
                <th><p>Latitude</p></th>
                <th><p>Longitude <br> other</p></th>
                <th><p>Depth <br> other</p></th>
                <th><p>Mag <br> other</p></th>
                <th><p>Location <br> other</p></th>
              </tr>
              <tr>
                <td style="width: 185px; height: 30px; " class="auto-style91">
                    <span class="auto-style7"></span><span class="auto-style70">
			            <a href="2020_Earthquake_Information\January\2019_1231_2132_B2.html">
			                <span class="auto-style99">01 January 2020 - 05:32 AM</span>
			            </a>
			        </span>
			    </td>
                <td>06.71</td>
                <td>126.21</td>
                <td>027</td>
                <td>2.4</td>
                <td style="width: 436px; height: 30px; " class="auto-style52">
		          022
		          <span style="font-family: &quot;Times New Roman&quot;, serif; font-size: 16px; font-style: normal; font-variant-ligatures: normal; font-variant-caps: normal; letter-spacing: normal; orphans: 2; text-align: start; text-indent: 0px; text-transform: none; white-space: normal; widows: 2; word-spacing: 0px; -webkit-text-stroke-width: 0px; background-color: rgb(255, 255, 255); text-decoration-style: initial; text-decoration-color: initial;" class="auto-style89">
		            <span class="auto-style78" style="font-size: 10pt;">
		                km N 47° E of Cabanglasan (Bukidnon)
		            </span>
		          </span>
		        </td>
              </tr>
            </table>
        "#;

    #[actix_rt::test]
    async fn parse_html_test() {
        let html = HTML.to_string();
        let result = parse_html(html).await;
        println!("{:?}", &result);
        assert_eq!(result[0]["Mag"], "2.4");
    }

    #[actix_rt::test]
    async fn parse_quakes_test() {
        let html = HTML.to_string();
        let data = parse_html(html).await;
        let quakes = parse_quakes(data).await.unwrap();
        println!("{:?}", &quakes);
        assert!(quakes.len() > 0);
    }

    #[actix_rt::test]
    async fn parse_real_quakes_test() {
        let html = retrieve_html().await.unwrap();
        let data = parse_html(html).await;
        println!("{:?}", data);
        assert!(data.len() > 0);
    }
}