use poise::CreateReply;
use crate::Error;
use crate::util::globals::{Mood, random_emote};

#[poise::command(slash_command, prefix_command)]
pub async fn stop(
    ctx: crate::Context<'_>,
) -> Result<(), Error> {

    let guild = &ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.as_ref())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.stop();

        let reply = CreateReply::default()
            .content(format!(":stop_button: Playlist stopped. {}", random_emote(Mood::Neutral)))
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
