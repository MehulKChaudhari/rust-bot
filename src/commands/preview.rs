use reqwest;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use serde_json::Value;
use serenity::{
    builder::{CreateEmbed, CreateMessage},
    model::{channel::Message, id::ChannelId},
    prelude::*,
};
use std::env;
use std::error::Error;
use url::Url;
use urlencoding::encode;

// Structure to hold link preview data
struct LinkPreviewData {
    title: Value,
    description: Value,
    image: Value,
    screenshot_url: String, // Added screenshot URL
}

// Fetch link preview data from an external Open Graph service
async fn fetch_link_preview_data(
    url: &str,
    api_key: &str,
) -> Result<LinkPreviewData, Box<dyn std::error::Error + Send>> {
    let encoded_url = encode(url);
    let og_url = format!(
        "https://opengraph.io/api/1.1/site/{}?app_id={}",
        encoded_url, api_key
    );
    let screenshot_url = format!(
        "https://opengraph.io/api/1.1/screenshot/{}?app_id={}&quality=60&dimensions=lg",
        encoded_url, api_key
    );

     let response = match reqwest::get(&og_url).await {
        Ok(resp) => resp,
        Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error + Send>),
    };

    // Read the response text
    let response_text = match response.text().await {
        Ok(text) => text,
        Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error + Send>),
    };

    // Parse JSON from the response text
    let json: serde_json::Value = match serde_json::from_str(&response_text) {
        Ok(value) => value,
        Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error + Send>),
    };

      let screenshot_response = match reqwest::get(&screenshot_url).await {
        Ok(resp) => resp,
        Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error + Send>),
    };

    // Read the screenshot API response text
    let screenshot_response_text = match screenshot_response.text().await {
        Ok(text) => text,
        Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error + Send>),
    };

    // Parse JSON from the screenshot API response text
    let screenshot_json: serde_json::Value = match serde_json::from_str(&screenshot_response_text) {
        Ok(value) => value,
        Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error + Send>),
    };

    // Get the screenshot URL from the screenshot API response
    let screenshot_url_from_api = screenshot_json
        .get("screenshotUrl")
        .and_then(|url| url.as_str())
        .map(String::from)
        .unwrap_or_default();

    Ok(LinkPreviewData {
        title: json
            .get("openGraph")
            .and_then(|og| og.get("title"))
            .cloned()
            .unwrap_or_default(),
        description: json
            .get("openGraph")
            .and_then(|og| og.get("description"))
            .cloned()
            .unwrap_or_default(),
        image: json
            .get("openGraph")
            .and_then(|og| og.get("image").and_then(|img| img.get("url")))
            .cloned()
            .unwrap_or_default(),
        screenshot_url: screenshot_url_from_api,
    })
}

pub async fn create_previews(ctx: &Context, msg: &Message) {
    dotenv::dotenv().ok();
    let api_key = env::var("OPENGRAPH_API_KEY").expect("OPENGRAPH_API_KEY not found in .env");

    for url in extract_urls_from_message(&msg.content) {
        if let Ok(preview_data) = fetch_link_preview_data(&url, &api_key).await {
            let embed = CreateEmbed::new()
                .title(preview_data.title.as_str().unwrap_or_default())
                .description(format!(
                    "{}\n\nFor more information, [click here]({})",
                    preview_data.description.as_str().unwrap_or_default(),
                    &url
                ))
                .thumbnail(preview_data.image.as_str().unwrap_or_default())
                .image(preview_data.screenshot_url.as_str())
                .color(0x3498db);
            
            let builder = CreateMessage::new().embed(embed);
            if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
                eprintln!("Error sending message: {:?}", why);
            };
        }
    }
}

// Extract URLs from the message content
fn extract_urls_from_message(content: &str) -> Vec<String> {
    content
        .split_whitespace()
        .filter(|&word| Url::parse(word).is_ok())
        .map(|url| url.to_string())
        .collect()
}
