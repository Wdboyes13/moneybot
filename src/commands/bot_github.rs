use poise::CreateReply;
use crate::library::cmdutil;

#[poise::command(slash_command, prefix_command)]
pub async fn github(ctx: cmdutil::Context<'_>) -> Result<(), cmdutil::Error> {
    ctx.send(CreateReply::default()
        .content("This bot is completely free & open source, code available @ https://github.weelam.ca/moneybot")
        .reply(true)
    ).await?;
    Ok(())
}