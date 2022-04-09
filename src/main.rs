mod config;

use crate::config::TOKEN;
use serenity::client::Client;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::client::EventHandler;
use serenity::async_trait;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "!ping" {
            let _ = msg.channel_id.say(&context, "Pong!").await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = String::from(TOKEN);
    
    let mut bot = Client
        ::builder(&token)
        .event_handler(Handler)
        .await.expect("error creating client");
    bot.start().await.expect("error starting bot");
}
