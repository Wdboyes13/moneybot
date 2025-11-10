use crate::library::botdb;
use crate::library::cmdutil;
use poise::CreateReply;

pub async fn bot_check(gid: u64, uid: u64) -> Result<u32, botdb::MoneyError> {
    cmdutil::safe_open_db!(gid, |db: botdb::MoneyDatabase| {
        if let Ok(money) = db.get_balance(uid) {
            return Ok(money);
        } else {
            println!("Error getting balance");
            return Ok(0);
        }
    })
}

#[poise::command(slash_command, prefix_command)]
pub async fn check(
    ctx: cmdutil::Context<'_>,
) -> Result<(), cmdutil::Error> {
    let guild_id = ctx.guild_id().expect("Command used in guild").get();
    let user_id = ctx.author().id.get();
    
    println!("Received check command from {}", ctx.author().name);
    match bot_check(guild_id, user_id).await {
        Ok(amount) => {
            ctx.send(CreateReply::default()
                .content(format!("You currently have ${}", amount))
                .reply(true)
            ).await?;
        },

        Err(err) => {
            let _ = cmdutil::send_err!(ctx, err);
        }
    }
    
    Ok(())
}