mod json;

use reqwest::Error;
use serde::Deserialize;
use dotenv::dotenv;
use std::fmt::{self, Display};
use json::display_json;

#[derive(Debug, Deserialize, Clone)]
struct WeatherReport {
    publishingOffice: String,
    reportDatetime: String,
    timeSeries: Vec<TimeSeries>,
}

#[derive(Debug, Deserialize, Clone)]
struct TimeSeries {
    #[serde(rename = "timeDefines")]
    timeDefines: Vec<String>,
    areas: Vec<Area>,
}

#[derive(Debug, Deserialize, Clone)]
struct Area {
    area: AreaDetail,
    #[serde(default)]
    weatherCodes: Vec<String>,
    #[serde(default)]
    weathers: Vec<String>,
    #[serde(default)]
    winds: Vec<String>,
    #[serde(default)]
    waves: Vec<String>,
    #[serde(default)]
    pops: Vec<String>,
    #[serde(default)]
    temps: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct AreaDetail {
    name: String,
    code: String,
}

impl Display for WeatherReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Publishing Office: {}", self.publishingOffice)?;
        writeln!(f, "Report Datetime: {}", self.reportDatetime)?;
        writeln!(f, "Time Series:")?;
        for (i, time_series) in self.timeSeries.iter().enumerate() {
            writeln!(f, "  [{}] Time Defines:", i)?;
            for time_define in &time_series.timeDefines {
                writeln!(f, "    {}", time_define)?;
            }
            writeln!(f, "  [{}] Areas:", i)?;
            for area in &time_series.areas {
                writeln!(f, "    Area: {} ({})", area.area.name, area.area.code)?;
                if !area.weatherCodes.is_empty() {
                    writeln!(f, "    Weather Codes: {:?}", area.weatherCodes)?;
                }
                if !area.weathers.is_empty() {
                    let sanitized_weathers: Vec<String> = area
                        .weathers
                        .iter()
                        .map(|weather| sanitize_unicode_spaces(weather))
                        .collect();
                    writeln!(f, "    Weathers: {:?}", sanitized_weathers)?;
                }
                if !area.winds.is_empty() {
                    let sanitized_winds: Vec<String> = area
                        .winds
                        .iter()
                        .map(|wind| sanitize_unicode_spaces(wind))
                        .collect();
                    writeln!(f, "    Winds: {:?}", sanitized_winds)?;
                }
                if !area.waves.is_empty() {
                    let sanitized_waves: Vec<String> = area
                        .waves
                        .iter()
                        .map(|wave| sanitize_unicode_spaces(wave))
                        .collect();
                    writeln!(f, "    Waves: {:?}", sanitized_waves)?;
                }
                if !area.pops.is_empty() {
                    writeln!(f, "    Pops: {:?}", area.pops)?;
                }
                if !area.temps.is_empty() {
                    writeln!(f, "    Temps: {:?}", area.temps)?;
                }
            }
        }
        Ok(())
    }
}

fn sanitize_unicode_spaces(input: &str) -> String {
    input.replace('\u{3000}', " ")
}

// ApiResponseをVec<WeatherReport>の型エイリアスとして定義
type ApiResponse = Vec<WeatherReport>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //geocode from environment variable
    dotenv().ok();
    let geocode = std::env::var("GEOCODE").expect("GEOCODE must be set");
    let base_url = "https://www.jma.go.jp/bosai/forecast/data/forecast/";
    let request_url = format!("{}{}.json", base_url, geocode);
    println!("Request URL: {}", request_url);
    
    // Send request and get the response text
    let response_text = reqwest::get(request_url)
        .await?
        .text()
        .await?;

    println!("{}", response_text.clone());
    println!("{:?}", display_json(response_text.clone()));
    // ここまでは動く、下の方でエラーが出る

    // Deserialize the response text to ApiResponse using match
    let api_response: ApiResponse = match serde_json::from_str(&response_text) {
        Ok(api_response) => api_response,
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    };

    let report = api_response[0].clone();

    println!("Weather information:\n{}", report);

    Ok(())

}
