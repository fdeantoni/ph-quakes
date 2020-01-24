use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use quakes_api::*;

use scraper::{Html, Selector, element_ref::Text};
use crate::ScraperError;

const DATETIME_FORMAT: &str = "%d %B %Y - %I:%M %p %#z";

type Row = HashMap<String, String>;

pub struct HtmlParser(Vec<Row>);

impl HtmlParser {

    pub async fn parse(html: String, source_url: String) -> HtmlParser {
        let mut collection: Vec<Row> = Vec::new();
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
                    let mut row = Row::new();
                    for (idx, td) in tr.select(&td_selector).enumerate() {
                        let idx_header = idx.to_string();
                        let header = table_headers.get(idx).unwrap_or(&idx_header).clone();
                        if header.eq("Date - Time") {
                            let url = td.select(&url_selector).last().unwrap().value().attr("href").unwrap_or("");
                            let url = url.replace("\\", "/");
                            row.insert("url".to_string(), format!("{}{}", source_url, url).to_string());
                            let text = Self::sanitize_text(td.text())
                                .last().cloned().unwrap_or("error");
                            row.insert("Date - Time".to_string(), format!("{} +08", text));
                        } else if header.eq("Location") {
                            let text: String = Self::sanitize_text(td.text()).join(" ");
                            let (location, province) = Self::find_province(text);
                            row.insert("Location".to_string(), location);
                            row.insert("Province".to_string(), province);
                        } else if header.eq("Mag") {
                            let text = Self::sanitize_text(td.text())
                                .last().cloned().unwrap_or("error");
                            row.insert(header, text.to_string());
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
        HtmlParser(collection)
    }

    fn sanitize_text(text: Text<'_>) -> Vec<&str> {
        text
            .into_iter().map(|string| string.trim() )
            .filter(|string| !string.is_empty() )
            .collect()
    }

    fn find_province(text: String) -> (String, String) {
        match text.rfind("(") {
            Some(pos) => {
                let len = text.len();
                let province = &text[pos+1..len-1];
                let location = &text[0..pos-1];
                (location.to_string(), province.to_string())
            },
            None => (text, String::new())
        }
    }

    fn get_datetime(row: &HashMap<String, String>) -> Result<DateTime<Utc>, ScraperError> {
        let text = row.get("Date - Time");
        if text.is_some() {
            let value = text.unwrap();
            let datetime: DateTime<FixedOffset> = DateTime::parse_from_str(value, DATETIME_FORMAT)
                .map_err(|error|{
                    ScraperError::new(format!("Trouble converting {} to timestamp: {}", value, error.to_string()))
                })?;
            let utc: DateTime<Utc> = DateTime::from(datetime);
            Ok(utc)
        } else {
            Err(ScraperError::new("Date - Time not found in row!"))
        }
    }

    fn get_longitude(row: &HashMap<String, String>) -> Result<f64, ScraperError> {
        let text = row.get("Longitude");
        if text.is_some() {
            let value = text.unwrap();
            value.parse::<f64>().map_err(|error| {
                ScraperError::new(format!("Trouble converting {} to f64: {}", value, error.to_string()))
            })
        } else {
            Err(ScraperError::new("Longitude not found in row!"))
        }
    }

    fn get_latitude(row: &HashMap<String, String>) -> Result<f64, ScraperError> {
        let text = row.get("Latitude");
        if text.is_some() {
            let value = text.unwrap();
            value.parse::<f64>().map_err(|error| {
                ScraperError::new(format!("Trouble converting {} to f64: {}", value, error.to_string()))
            })
        } else {
            Err(ScraperError::new("Latitude not found in row!"))
        }
    }

    fn get_magnitude(row: &HashMap<String, String>) -> Result<f64, ScraperError> {
        let text = row.get("Mag");
        if text.is_some() {
            let value = text.unwrap();
            value.parse::<f64>().map_err(|error| {
                ScraperError::new(format!("Trouble converting {} to f64: {}", value, error.to_string()))
            })
        } else {
            Err(ScraperError::new("Mag not found in row!"))
        }
    }

    fn get_depth(row: &HashMap<String, String>) -> Result<u16, ScraperError> {
        let text = row.get("Depth");
        if text.is_some() {
            let value = text.unwrap();
            value.parse::<u16>().map_err(|error| {
                ScraperError::new(format!("Trouble converting {} to i8: {}", value, error.to_string()))
            })
        } else {
            Err(ScraperError::new("Depth not found in row!"))
        }
    }

    fn get_location(row: &HashMap<String, String>) -> Result<String, ScraperError> {
        let text = row.get("Location");
        if text.is_some() {
            Ok(text.unwrap().clone())
        } else {
            Err(ScraperError::new("Location not found in row!"))
        }
    }

    fn get_province(row: &HashMap<String, String>) -> Result<String, ScraperError> {
        let text = row.get("Province");
        if text.is_some() {
            Ok(text.unwrap().clone())
        } else {
            Err(ScraperError::new("province not found in row!"))
        }
    }

    fn get_url(row: &HashMap<String, String>) -> Result<String, ScraperError> {
        let text = row.get("url");
        if text.is_some() {
            Ok(text.unwrap().clone())
        } else {
            Err(ScraperError::new("URL not found in row!"))
        }
    }

    pub async fn get_quakes(&self) -> Result<Vec<Quake>, ScraperError> {
        let mut quakes = Vec::new();

        for row in self.0.clone() {

            let datetime = Self::get_datetime(&row)?;
            let longitude = Self::get_longitude(&row)?;
            let latitude = Self::get_latitude(&row)?;
            let magnitude = Self::get_magnitude(&row)?;
            let depth = Self::get_depth(&row)?;
            let location = Self::get_location(&row)?;
            let province = Self::get_province(&row)?;
            let url = Self::get_url(&row)?;

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
        }
        Ok(quakes)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

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
                <td><strong>4.0</strong></td>
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
    async fn parse_html() {
        let parser = HtmlParser::parse(HTML.to_string(), "http://www.example.com/".to_string()).await;
        let quakes = parser.get_quakes().await.unwrap();
        println!("{:?}", &quakes);
        assert_eq!(quakes[0].get_magnitude(), 4.0);
    }

    #[test]
    fn datetime_parsing() {
        let mut row: Row = Row::new();
        let header = "Date - Time".to_string();
        let dt_string =  "22 January 2020 - 12:40 AM +08".to_string();
        row.insert(header, dt_string);
        let dt = HtmlParser::get_datetime(&row).unwrap();
        println!("{:?}", dt);
    }

    #[test]
    fn parse_province() {
        let text = "10km N 47° E of Cabanglasan (Bukidnon)".to_string();
        let (location, province) = HtmlParser::find_province(text);
        println!("{:?}", location);
        println!("{:?}", province);
    }

}