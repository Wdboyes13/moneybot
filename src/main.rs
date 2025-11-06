use std::env;

use serenity::{Client, all::{Context, EventHandler, GatewayIntents, Message}, async_trait};

mod bot_work;
mod bot_check;
mod bot_gamble;
mod botdb;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.guild_id.is_some() {
            if msg.content == "!work" {
                println!("Received work command from {}", msg.author.display_name());
                let amnt = bot_work::bot_work(
                    u64::from(msg.guild_id.expect("is_some was true, but there was no value")), 
                    u64::from(msg.author.id)
                );

                if let Err(why) = msg.reply_ping(&ctx.http, format!("You earned {amnt:?}")).await {
                    println!("Error sending message: {why:?}");
                }
            } else if msg.content == "!check" {
                println!("Received check command from {}", msg.author.display_name());
                let amnt = bot_check::bot_check(
                    u64::from(msg.guild_id.expect("is_some was true, but there was no value")), 
                    u64::from(msg.author.id)
                );

                if let  Err(why) = msg.reply_ping(&ctx.http, format!("You currently have {amnt:?}")).await {
                    println!("Error sending message: {why:?}");
                }
            } else if msg.content.starts_with("!gamble") {
                println!("Received gamble command from {}", msg.author.display_name());
                if let Some(amount) = msg.content.get(7..) {
                    if let Ok(nmount) = amount.trim().parse::<u32>() {
                        let amnt = bot_gamble::bot_gamble(
                            u64::from(msg.guild_id.expect("is_some was true, but there was no value")), 
                            u64::from(msg.author.id),
                            nmount
                        );

                        if let Err(why) = msg.reply_ping(&ctx.http, amnt).await {
                            println!("Error sending message: {why:?}");
                        }
                    } else {
                        if let Err(why) = msg.reply_ping(&ctx.http, "Missing parameter amount").await {
                            println!("Error sending message: {why:?}");
                        }
                    }
                } else {
                    if let Err(why) = msg.reply_ping(&ctx.http, "Missing parameter amount").await {
                            println!("Error sending message: {why:?}");
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("MONEYBOT_TOKEN")
        .expect("Discord bot token must be set using MONEYBOT_TOKEN environment variable");

    let intents 
        = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client Error: {why:?}");
    }
}