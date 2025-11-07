use crate::botdb;
use std::vec::Vec;
use serenity::all::{Context, GuildId};

pub struct MoneyLeaderboard {
    pub number: u32,
    pub users: Vec::<u32>,
    pub balances: Vec::<u32>
}

pub fn get_top_users(gid: u64) -> Result<MoneyLeaderboard, botdb::MoneyError> {
    if let Ok(db) = botdb::MoneyDatabase::open(gid) {
        let mut stmt = db.conn.prepare(botdb::tblfmt!(db, "SELECT uid, balance FROM {} ORDER BY balance DESC LIMIT 3"))?;
        let mut rows = stmt.query([])?;
        let mut board = MoneyLeaderboard {
            number: 3,
            users: Vec::new(),
            balances: Vec::new(),
        };
        
        while let Some(row) = rows.next()? {
            let uid: u32 = row.get(0)?;
            let balance: u32 = row.get(1)?;
            board.users.push(uid);
            board.balances.push(balance);
        }
        
        Ok(board)
    } else {
        panic!("ERROR: Unable to open SQLITE database");
    }
}

pub async fn bot_top10(gid: u64, ctx: &Context) -> Result<String, botdb::MoneyError> {
    
    let (users, balances, number) = {
        let board = get_top_users(gid)?;
        let users = board.users.iter().map(|&id| id as u64).collect::<Vec<u64>>();
        let balances = board.balances.clone();
        let number = board.number;
        (users, balances, number)
    };

    let user_ids: Vec<serenity::all::UserId> = users.into_iter()
        .map(serenity::all::UserId::from)
        .collect();
        
    let http = ctx.http.clone();
    let mut names = Vec::<String>::new();

    for (_, uid) in user_ids.iter().take(number as usize).enumerate() {
        let id = *uid;

        match http.get_member(GuildId::from(gid), id).await {
            Ok(user) => names.push(user.display_name().to_string()),
            Err(why) => {
                println!("Error fetching user {}: {why:?}", uid);
                names.push(format!("User_{}", uid));
            }
        }
    }

    Ok((0..number)
        .map(|i| {
            format!(
                "{}. {} - ${}",
                i + 1,
                names.get(i as usize).unwrap_or(&"Unknown".to_string()),
                balances.get(i as usize).unwrap_or(&0)
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
    )
}    
       