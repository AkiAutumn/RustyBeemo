use crate::Context;
use crate::Error;
use poise::serenity_prelude as serenity;

/// Returns a URL to the user's avatar 
#[poise::command(slash_command, prefix_command)]
pub async fn steal_profile_pic(
    ctx: Context<'_>,
    #[description = "Select user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    ctx.reply(u.avatar_url().unwrap()).await?;
    Ok(())
}