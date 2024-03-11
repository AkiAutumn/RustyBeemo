use std::process;
use std::process::Command;
use execute::Execute;
use poise::CreateReply;
use crate::Context;
use crate::Error;
use crate::util::globals::{is_privileged_user, Mood, random_emote};

/// Pull the latest changes from git and restart the bot
#[poise::command(slash_command, prefix_command)]
pub async fn self_update(
    ctx: Context<'_>
) -> Result<(), Error> {

    if !is_privileged_user(ctx.author().id) {
        
        let response = format!("You're not allowed to do that! {}", random_emote(Mood::Negative));
        ctx.reply(response).await?;
        
    } else {
        
        let reply = CreateReply::default()
            .content(format!(":wrench: Updating myself... {}", random_emote(Mood::Positive)))
            .ephemeral(true);
        
        ctx.send(reply).await?;
        
        println!("Self-update requested by {}", ctx.author().name);
        
        let mut command = Command::new("./update.sh");

        if let Some(exit_code) = command.execute().unwrap() {
            if exit_code == 0 {
                println!("Exiting the process...");
                process::exit(0x0100);
            } else {
                eprintln!("Update failed!");
            }
        } else {
            eprintln!("Update Interrupted!");
        }
    }
    
    Ok(())
}