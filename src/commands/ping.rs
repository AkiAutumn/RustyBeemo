use crate::Context;
use crate::Error;

/// Returns my ping duration
#[poise::command(slash_command, prefix_command)]
pub async fn ping(
    ctx: Context<'_>
) -> Result<(), Error> {

    let response = format!("{} ms", ctx.ping().await.as_millis());

    ctx.reply(response).await?;
    Ok(())
}