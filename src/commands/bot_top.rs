use crate::library::botdb;
use std::vec::Vec;
use rusqlite::params;
use serenity::all::{Context, GuildId};
use crate::library::cmdutil;
use poise::CreateReply;

pub struct MoneyLeaderboard {
    pub users: Vec::<u64>,
    pub balances: Vec::<u32>
}

pub fn get_top_users(gid: u64, amnt: u32) -> Result<MoneyLeaderboard, botdb::MoneyError> {
    if let Ok(db) = botdb::MoneyDatabase::open(gid) {
        let mut stmt = db.conn.prepare(botdb::tblfmt!(db, "SELECT uid, balance FROM {} ORDER BY balance DESC LIMIT ?1"))?;
        let mut rows = stmt.query(params![amnt])?;
        let mut board = MoneyLeaderboard {
            users: Vec::new(),
            balances: Vec::new(),
        };
        
        while let Some(row) = rows.next()? {
            let uid: u64 = row.get(0)?;
            let balance: u32 = row.get(1)?;
            board.users.push(uid);
            board.balances.push(balance);
        }
        
        Ok(board)
    } else {
        panic!("ERROR: Unable to open SQLITE database");
    }
}

pub async fn bot_top(gid: u64, amnt: u32, ctx: &Context) -> Result<String, botdb::MoneyError> {
    
    let (users, balances) = {
        let board = get_top_users(gid, amnt)?;
        let users = board.users.iter().map(|&id| id as u64).collect::<Vec<u64>>();
        let balances = board.balances.clone();
        (users, balances)
    };

    let user_ids: Vec<serenity::all::UserId> = users.clone().into_iter()
        .map(serenity::all::UserId::from)
        .collect();
        
    let http = ctx.http.clone();
    let mut names = Vec::<String>::new();

    for uid in user_ids.iter() {
        let id = *uid;

        match http.get_member(GuildId::from(gid), id).await {
            Ok(user) => names.push(user.display_name().to_string()),
            Err(why) => {
                println!("Error fetching user {}: {why:?}", uid);
                names.push(format!("User_{}", uid));
            }
        }
    }

    Ok((names.iter().enumerate())
        .map(|(i, name)| {
            if let Some(balance) = balances.get(i) {
                format!("{}. {} - ${}", i + 1, name, balance)
            } else {
                String::new()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
    )
}    
       

#[poise::command(slash_command, prefix_command)]
pub async fn top(
    ctx: cmdutil::Context<'_>,
    #[description = "Number of users to show"] count: Option<u32>,
) -> Result<(), cmdutil::Error> {
    let guild_id = ctx.guild_id().expect("Command used in guild").get();
    let count = count.unwrap_or(10); // Default to top 10
    
    println!("Received top command from {}", ctx.author().name);
    let result = bot_top(guild_id, count, ctx.serenity_context()).await?;
    
    ctx.send(CreateReply::default()
        .content(result)
        .reply(true)
    ).await?;
    
    Ok(())
}