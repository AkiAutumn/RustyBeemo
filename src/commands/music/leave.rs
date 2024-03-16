use poise::CreateReply;
use crate::Error;
use crate::util::globals::{Mood, random_emote};

#[poise::command(slash_command, prefix_command)]
pub async fn leave(
    ctx: crate::Context<'_>,
) -> Result<(), Error> {
    
    let guild_id = ctx.guild().unwrap().id;

    let manager = songbird::get(ctx.as_ref())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            println!("Failed to leave voice channel: {}", e);
            
            let reply = CreateReply::default()
                .content(format!("Failed to leave voice channel... {}", random_emote(Mood::Negative)))
                .ephemeral(true);
            ctx.send(reply).await?;
        }

        let reply = CreateReply::default()
            .content(format!("I'm leaving... {}", random_emote(Mood::Negative)))
            .ephemeral(true);
        ctx.send(reply).await?;
        
    } else {
        let reply = CreateReply::default()
            .content(format!("You need to be in a voice channel! {}", random_emote(Mood::Negative)))
            .ephemeral(true);
        ctx.send(reply).await?;
    }

    Ok(())
}
