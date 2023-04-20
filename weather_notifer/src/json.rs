use serde::{Deserialize, Serialize};
use serde_json;
pub fn display_json(text: String) -> Result<(), serde_json::Error> {
    let json_str = text;
    // Pretty print JSON
    pretty_print_json(json_str.as_str())?;

    // Deserialize JSON
    let api_response: ApiResponse = serde_json::from_str(json_str.as_str())?;
    println!("Deserialized JSON data:\n{:#?}", api_response);

    Ok(())
}

fn pretty_print_json(json_str: &str) -> Result<(), serde_json::Error> {
    let json_value: serde_json::Value = serde_json::from_str(json_str)?;
    let pretty_json_str = serde_json::to_string_pretty(&json_value)?;
    println!("{}", pretty_json_str);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    publishingOffice: String,
    reportDatetime: String,
    timeSeries: Vec<TimeSeries>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimeSeries {
    timeDefines: Vec<String>,
    areas: Vec<AreaData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AreaData {
    area: Area,
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

#[derive(Debug, Serialize, Deserialize)]
struct Area {
    name: String,
    code: String,
}
