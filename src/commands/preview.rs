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
}

// Fetch link preview data from an external Open Graph service
async fn fetch_link_preview_data(
    url: &str,
    api_key: &str,
) -> Result<LinkPreviewData, Box<dyn std::error::Error + Send>> {
    // Encode the URL
    let encoded_url = encode(url);

    // Construct the API endpoint with the encoded URL
    let og_url = format!(
        "https://opengraph.io/api/1.1/site/{}?app_id={}",
        encoded_url, api_key
    );

    // Make the request
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
    })
}

pub async fn create_previews(ctx: &Context, msg: &Message) {
    // Load API key from .env file
    dotenv::dotenv().ok();
    let api_key = env::var("OPENGRAPH_API_KEY").expect("OPENGRAPH_API_KEY not found in .env");

    for url in extract_urls_from_message(&msg.content) {
        if let Ok(preview_data) = fetch_link_preview_data(&url, &api_key).await {
            // Create an embed with the link information
            let embed = CreateEmbed::new()
                .title(preview_data.title.as_str().unwrap_or_default())
                .description(preview_data.description.as_str().unwrap_or_default())
                .url(&url)
                .thumbnail(preview_data.image.as_str().unwrap_or_default())
                .image(preview_data.image.as_str().unwrap_or_default())
                .color(0x3498db);

            // Send the embed to the channel
            let builder = CreateMessage::new().embed(embed);
            let message = msg.channel_id.send_message(&ctx.http, builder).await;
            if let Err(why) = message {
                eprintln!("Error sending message: {why:?}");
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
