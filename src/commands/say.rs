use crate::Context;
use crate::Error;
/// Sends a message containing the user's input text
#[poise::command(slash_command, prefix_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Message"] message: String,
) -> Result<(), Error> {
    
    ctx.say(message).await?;
    Ok(())
}