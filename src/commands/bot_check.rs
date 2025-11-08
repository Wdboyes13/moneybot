use crate::library::botdb;
use crate::library::cmdutil;
use poise::CreateReply;

pub fn bot_check(gid: u64, uid: u64) -> u32 {
    if let Ok(db) = botdb::MoneyDatabase::open(gid) {
        if let Ok(money) = db.get_balance(uid) {
            return money;
        } else {
            println!("Error getting balance");
            return 0;
        }
    } else {
        panic!("ERROR: Unable to open SQLITE database, is ~/.moneybot a directory?");
    }
}

#[poise::command(slash_command, prefix_command)]
pub async fn check(
    ctx: cmdutil::Context<'_>,
) -> Result<(), cmdutil::Error> {
    let guild_id = ctx.guild_id().expect("Command used in guild").get();
    let user_id = ctx.author().id.get();
    
    println!("Received check command from {}", ctx.author().name);
    let amount = bot_check(guild_id, user_id);
    
    ctx.send(CreateReply::default()
        .content(format!("You currently have ${}", amount))
        .reply(true)
    ).await?;
    
    Ok(())
}