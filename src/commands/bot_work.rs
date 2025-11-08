use rand::Rng;
use crate::library::botdb;
use crate::library::cmdutil;
use poise::CreateReply;

cmdutil::sec_check!(work_check, 3);

pub fn bot_work(gid: u64, uid: u64) -> u32 {
    let mut rng = rand::rng();
    let randint: u32 = rng.random_range(1..=1000);
    if let Ok(db) = botdb::MoneyDatabase::open(gid) {
        if let Err(why) = db.add_money(uid, randint) {
            println!("Error adding money: {why:?}");
            return 0;
        }
    } else {
        panic!("ERROR: Unable to open SQLITE database, is ~/.moneybot a directory?");
    }
    randint
}

#[poise::command(slash_command, prefix_command, check = work_check)]
pub async fn work(
    ctx: cmdutil::Context<'_>,
) -> Result<(), cmdutil::Error> {
    let guild_id = ctx.guild_id().expect("Command used in guild").get();
    let user_id = ctx.author().id.get();
    
    println!("Received work command from {}", ctx.author().name);
    let amount = bot_work(guild_id, user_id);
    
    ctx.send(CreateReply::default()
        .content(format!("You earned ${}", amount))
        .reply(true)
    ).await?;
    
    Ok(())
}