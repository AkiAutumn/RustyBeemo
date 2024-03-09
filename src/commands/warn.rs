use crate::Context;
use crate::Error;
use poise::serenity_prelude as serenity;
use serenity::prelude::Mentionable;

/// Warns a user who misbehaved
#[poise::command(slash_command, prefix_command)]
pub async fn warn(
    ctx: Context<'_>,
    #[description = "Select user"] user: Option<serenity::User>,
    #[description = "Specify reason"] reason: Option<String>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}' you have been warned by a moderator. Reason: {}", u.mention(), reason.unwrap());

    ctx.reply(response).await?;
    Ok(())
}