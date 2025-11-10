use crate::library::botdb;
use crate::library::cmdutil;
use poise::CreateReply;

pub async fn bot_transfer(gid: u64, from_uid: u64, to_uid: u64, amount: u32) -> Result<(), botdb::MoneyError> {
    cmdutil::safe_open_db!(gid, |db: botdb::MoneyDatabase| {
        db.delete_money(from_uid, amount)?;
        db.add_money(to_uid, amount)?;
        Ok(())
    })
}

#[poise::command(slash_command, prefix_command)]
pub async fn transfer(
    ctx: cmdutil::Context<'_>,
    #[description = "User to transfer to"] to: serenity::model::user::User,
    #[description = "Amount to transfer"] amnt: u32
) -> Result<(), cmdutil::Error> {
    let guild_id = ctx.guild_id().expect("Command used in guild").get();
    let user_id = ctx.author().id.get();

    if let Err(err) = bot_transfer(
        guild_id,
        user_id,
        to.id.into(),
        amnt
    ).await {
            ctx.send(
                CreateReply::default()
                .content(err.to_string())
                .reply(true)
            ).await?;
    } else {
        ctx.send(
            CreateReply::default()
            .content(
                format!(
                    "Succesfully transffered ${} from {} to {}", 
                    amnt, 
                    ctx.author().display_name(), 
                    to.display_name()
                )
            )
        ).await?;
    }

    Ok(())
}