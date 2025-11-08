use serenity::{ Client, all::GatewayIntents };
use poise::CreateReply;
use std::{env, collections::HashMap, sync::Arc, time::{Instant, Duration}};
use tokio::sync::RwLock;

mod bot_work;
mod bot_check;
mod bot_gamble;
mod bot_top;
mod botdb;

pub struct Data {
    command_usage: Arc<RwLock<HashMap<(u64, String), Instant>>>,
}

impl Data {
    fn new() -> Self {
        Self {
            command_usage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

async fn check_dr(ctx: Context<'_>, duration: Duration) -> Result<bool, Error> {
    let command_name = ctx.command().qualified_name.clone();
    let user_id = ctx.author().id.get();

    let mut usage = ctx.data().command_usage.write().await;
    let now = Instant::now();

    if let Some(last_used) = usage.get(&(user_id, command_name.clone())) {
        if now.duration_since(*last_used) < duration {
            ctx.say(format!("⏳ Please wait {} seconds before using this command again", duration.as_secs())).await?;
            return Ok(false);
        }
    }

    usage.insert((user_id, command_name), now);
    Ok(true)
}

macro_rules! sec_check {
    ($name:ident, $secs:literal) => {
        async fn $name(ctx: Context<'_>) -> Result<bool, Error> {
            check_dr(ctx, Duration::from_secs( $secs )).await
        }
    };
}

sec_check!(work_check, 3);
sec_check!(gamble_check, 20);

#[poise::command(slash_command, prefix_command, check = work_check)]
async fn work(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Command used in guild").get();
    let user_id = ctx.author().id.get();
    
    println!("Received work command from {}", ctx.author().name);
    let amount = bot_work::bot_work(guild_id, user_id);
    
    ctx.send(CreateReply::default()
        .content(format!("You earned ${}", amount))
        .reply(true)
    ).await?;
    
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn check(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Command used in guild").get();
    let user_id = ctx.author().id.get();
    
    println!("Received check command from {}", ctx.author().name);
    let amount = bot_check::bot_check(guild_id, user_id);
    
    ctx.send(CreateReply::default()
        .content(format!("You currently have ${}", amount))
        .reply(true)
    ).await?;
    
    Ok(())
}

#[poise::command(slash_command, prefix_command, check = gamble_check)]
async fn gamble(
    ctx: Context<'_>,
    #[description = "Amount to gamble"] amount: u32,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Command used in guild").get();
    let user_id = ctx.author().id.get();
    
    println!("Received gamble command from {}", ctx.author().name);
    let result = bot_gamble::bot_gamble(guild_id, user_id, amount);
    
    ctx.send(CreateReply::default()
        .content(result)
        .reply(true)
    ).await?;
    
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn top(
    ctx: Context<'_>,
    #[description = "Number of users to show"] count: Option<u32>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Command used in guild").get();
    let count = count.unwrap_or(10); // Default to top 10
    
    println!("Received top command from {}", ctx.author().name);
    let result = bot_top::bot_top(guild_id, count, ctx.serenity_context()).await?;
    
    ctx.send(CreateReply::default()
        .content(result)
        .reply(true)
    ).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = env::var("MONEYBOT_TOKEN")
        .expect("Discord bot token must be set using MONEYBOT_TOKEN environment variable");
    let data = Data::new();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![work(), check(), gamble(), top()],
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