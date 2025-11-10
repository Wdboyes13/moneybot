use std::{collections::HashMap, sync::Arc, time::{Instant, Duration}};
use tokio::sync::RwLock;

pub struct Data {
    command_usage: Arc<RwLock<HashMap<(u64, String), Instant>>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            command_usage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn check_dr(ctx: Context<'_>, duration: Duration) -> Result<bool, Error> {
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
        async fn $name(ctx: crate::library::cmdutil::Context<'_>) 
            -> Result<bool, crate::library::cmdutil::Error> {
                crate::library::cmdutil::check_dr(ctx, std::time::Duration::from_secs( $secs )).await
        }
    };
}

pub(crate) use sec_check;


macro_rules! send_err {
    ($ctx:ident, $err:ident) => {
        $ctx.send(CreateReply::default()
            .content($err.to_string())
            .reply(true)
        ).await
    }
}

pub(crate) use send_err;


macro_rules! safe_open_db {
    ($gid:ident, $dbops:expr) => {
        match botdb::MoneyDatabase::open($gid) {
            Ok(db) => $dbops(db),
            Err(err) => return Err(err.into()),
        }
    }
}

pub(crate) use safe_open_db;