use axum::{routing::get, Router};
use axum::response::IntoResponse;
use reqwest::{Client, StatusCode};
use chrono::{DateTime, Local};
use std::net::SocketAddr;
use anyhow::Result;
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct WeatherOverview {
    publishingOffice: String,
    reportDatetime: String,
    targetArea: String,
    headlineText: String,
    text: String,
}

async fn getOverview() -> Result<WeatherOverview> {

    let client = Client::new();
    let jma_url = "https://www.jma.go.jp/bosai/forecast/data/overview_forecast/230000.json";

    let response= client
        .get(jma_url)
        .send()
        .await?;
    let body = response
        .json::<WeatherOverview>()
        .await?;
    Ok(body)
}

async fn sendToDiscord(overview: WeatherOverview) -> Result<()>{

    let local_datetime: DateTime<Local> = Local::now();
    let client = Client::new();
    let dis_url = "your webhook url";
    let overview = overview.text;
    let embed_text = serde_json::json!({
        "username": "Ageha",
        "avatar_url": "https://github.com/raiga0310.png",
        "content": "weather overview",
        "embeds": [
            {
                "title": "Overview",
                "description": format!("{:#?}", overview),
                "url": "https://www.jma.go.jp/bosai/#pattern=forecast",
                "timestamp": local_datetime.to_rfc3339(),
                "footer": {
                    "text": "Ageha Weather Notification"
                },
            }
        ]
    }
    );

    let result = client
        .post(dis_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(embed_text.to_string())
        .timeout(core::time::Duration::from_secs(10))
        .send()
        .await;
    Ok(())
}

async fn root() -> impl IntoResponse {
    match getOverview().await {
        Ok(overview) => match
            sendToDiscord(overview).await {
            Ok(()) => (StatusCode::OK, "sent"),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, "failed to send discord")
        },

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, "failed to fetch weather overview")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
