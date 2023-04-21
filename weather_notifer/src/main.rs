mod weather;

use axum::{routing::get, Router, response::IntoResponse, http::StatusCode};
use dotenv::dotenv;
use std::net::SocketAddr;
use weather::ApiResponse;
// forecast handler
async fn forecast_handler() -> Result<impl IntoResponse, StatusCode> {
    dotenv().ok();
    let geocode = std::env::var("GEOCODE").expect("GEOCODE must be set");
    let base_url = "https://www.jma.go.jp/bosai/forecast/data/forecast/";
    let request_url = format!("{}{}.json", base_url, geocode);

    // Send request and get the response text
    let response_text = reqwest::get(request_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // Deserialize the response text to ApiResponse using match
    let api_response: ApiResponse = serde_json::from_str(&response_text).unwrap();

    let report = api_response[0].clone();

    Ok((StatusCode::OK, report.display_weather_and_temperature()))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/forecast", get(forecast_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
