use rand::Rng;
use crate::library::botdb;
use crate::library::cmdutil;
use poise::CreateReply;

cmdutil::sec_check!(gamble_check, 20);

pub fn bot_gamble(gid: u64, uid: u64, amount: u32) -> String {
    let mut rng = rand::rng();
    let mut avg: u64 = 0;
    for _ in 1..=10 {
        avg += rng.random_range(1..=10);
    }
    avg /= 10;

    if let Ok(db) = botdb::MoneyDatabase::open(gid) {
        if let Err(err) = db.delete_money(uid, amount) {
            return err.to_string();
        }
        if avg > 5 {
            if let Ok(()) = db.add_money(uid, amount * 2) {
                return format!("You won {}", amount * 2).to_string();
            } else {
                return "There was an error".to_string();
            }
        } else {
            return format!("You lost {}", amount);
        }
    } else {
        panic!("ERROR: Unable to open SQLITE database, is ~/.moneybot a directory?");
    }
}

#[poise::command(slash_command, prefix_command, check = gamble_check)]
pub async fn gamble(
    ctx: cmdutil::Context<'_>,
    #[description = "Amount to gamble"] amount: u32,
) -> Result<(), cmdutil::Error> {
    let guild_id = ctx.guild_id().expect("Command used in guild").get();
    let user_id = ctx.author().id.get();
    
    println!("Received gamble command from {}", ctx.author().name);
    let result = bot_gamble(guild_id, user_id, amount);
    
    ctx.send(CreateReply::default()
        .content(result)
        .reply(true)
    ).await?;
    
    Ok(())
}