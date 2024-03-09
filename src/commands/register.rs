use crate::{Context, Error};
use crate::util::globals::{is_privileged_user, Mood, random_emote};

/// Creates a message with buttons for (un)-registering application commands
#[poise::command(slash_command, prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    
    if is_privileged_user(ctx.author().id) {
        poise::builtins::register_application_commands_buttons(ctx).await?;
    } else {
        let response = format!("You're not allowed to do that! {}", random_emote(Mood::Negative));
        ctx.reply(response).await?;
    }
    
    Ok(())
}