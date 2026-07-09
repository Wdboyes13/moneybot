use rand::Rng;
use crate::library::botdb;
use crate::library::cmdutil;
use poise::CreateReply;

cmdutil::sec_check!(gamble_check, 20);

pub async fn bot_gamble(gid: u64, uid: u64, amount: u32) -> Result<String, botdb::MoneyError> {
    let mut rng = rand::rng();
    let rand: u64 = rng.random_range(1..=100);

  
    cmdutil::safe_open_db!(gid, |db: botdb::MoneyDatabase| {
        if let Err(err) = db.delete_money(uid, amount) {
            return Err(err);
        }
        if rand > 48 {
            if let Ok(()) = db.add_money(uid, amount * 2) {
                return Ok(format!("You won {}", amount * 2).to_string());
            } else {
                return Ok("There was an error".to_string());
            }
        } else {
            return Ok(format!("You lost {}", amount));
        }
    })
}

#[poise::command(slash_command, prefix_command, check = gamble_check)]
pub async fn gamble(
    ctx: cmdutil::Context<'_>,
    #[description = "Amount to gamble"] amount: u32,
) -> Result<(), cmdutil::Error> {
    let guild_id = ctx.guild_id().expect("Command used in guild").get();
    let user_id = ctx.author().id.get();
    
    println!("Received gamble command from {}", ctx.author().name);
    match bot_gamble(guild_id, user_id, amount).await {
        Ok(result) => {
            ctx.send(CreateReply::default()
                .content(result)
                .reply(true)
            ).await?;
        },

        Err(err) => {
            let _ = cmdutil::send_err!(ctx, err);
        }
    }
    
    
    
    Ok(())
}