use serenity::{ Client, all::GatewayIntents };
use std::env;

pub mod library;
pub mod commands;

#[tokio::main]
async fn main() {
    let token = env::var("MONEYBOT_TOKEN")
        .expect("Discord bot token must be set using MONEYBOT_TOKEN environment variable");
    let data = library::cmdutil::Data::new();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                    commands::work(), 
                    commands::check(), 
                    commands::gamble(), 
                    commands::top(),
                    commands::github(),
                    commands::transfer()
                ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("Registering commands...");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let intents 
        = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client Error: {why:?}");
    }
}