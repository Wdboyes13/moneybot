use rand::Rng;
use crate::library::botdb;
use crate::library::cmdutil;
use poise::CreateReply;

cmdutil::sec_check!(work_check, 3);

pub fn bot_work(gid: u64, uid: u64) -> Result<u32, botdb::MoneyError> {
    let mut rng = rand::rng();
    let randint: u32 = rng.random_range(1..=1000);
    cmdutil::safe_open_db!(gid, |db: botdb::MoneyDatabase| {
        if let Err(why) = db.add_money(uid, randint) {
            println!("Error adding money: {why:?}");
            return Ok(0);
        }
        return Ok(randint);
    })
}

#[poise::command(slash_command, prefix_command, check = work_check)]
pub async fn work(
    ctx: cmdutil::Context<'_>,
) -> Result<(), cmdutil::Error> {
    let guild_id = ctx.guild_id().expect("Command used in guild").get();
    let user_id = ctx.author().id.get();
    
    println!("Received work command from {}", ctx.author().name);
    match bot_work(guild_id, user_id) {
        Ok(amount) => {
            ctx.send(CreateReply::default()
                .content(format!("You earned ${}", amount))
                .reply(true)
            ).await?;
        },

        Err(err) => {
            let _ = cmdutil::send_err!(ctx, err);
        }
    }
    
    
    Ok(())
}