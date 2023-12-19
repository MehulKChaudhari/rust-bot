use dotenv::dotenv;
use std::env;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

mod commands;

const HELP_COMMAND: &str = "!help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            commands::help::help_command(&ctx, &msg).await;
            println!("Help command executed!");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
