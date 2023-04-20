mod weather;

use dotenv::dotenv;
use reqwest::Error;
use weather::ApiResponse;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //geocode from environment variable
    dotenv().ok();
    let geocode = std::env::var("GEOCODE").expect("GEOCODE must be set");
    let base_url = "https://www.jma.go.jp/bosai/forecast/data/forecast/";
    let request_url = format!("{}{}.json", base_url, geocode);
    println!("Request URL: {}", request_url);

    // Send request and get the response text
    let response_text = reqwest::get(request_url).await?.text().await?;

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
