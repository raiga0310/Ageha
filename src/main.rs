use chrono::{Local, DateTime};
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WeatherOverview {
    publishingOffice: String,
    reportDatetime: String,
    targetArea: String,
    headlineText: String,
    text: String,
}

#[tokio::main]
async fn main() -> Result<()>{
    println!("天気なんざ自分で調べろ");

    let local_datetime: DateTime<Local> = Local::now();
    println!("{}", local_datetime);

    let client = Client::new();
    let jma_url = "https://www.jma.go.jp/bosai/forecast/data/overview_forecast/230000.json";

    let response = client
        .get(jma_url)
        .send()
        .await?;
    let body = response.json::<WeatherOverview>().await?;
    println!("{:#?}", body);
    Ok(())
}
