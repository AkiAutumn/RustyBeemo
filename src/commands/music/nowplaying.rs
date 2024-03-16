use futures_util::TryFutureExt;
use poise::CreateReply;
use crate::Error;
use serenity::model::prelude::*;
use crate::util::globals::{Mood, random_emote};
use crate::util::utils::to_time;

#[poise::command(slash_command, prefix_command)]
pub async fn now_playing(
    ctx: crate::Context<'_>,
) -> Result<(), Error> {
    /*
    let guild = &ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.as_ref())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        let current = match queue.current() {
            Some(current) => current,
            None => {
                let reply = CreateReply::default()
                    .content(format!("It's quiet for now... {}", random_emote(Mood::Neutral)))
                    .ephemeral(true);
                ctx.send(reply).await?;

                return Ok(());
            }
        };
        
        let metadata = current.metadata();
        let track_info = current.get_info().await.unwrap();

        let date_formatted = match &metadata.date {
            Some(date) => {
                format!("{}/{}/{}", &date[6..8], &date[4..6], &date[0..4])
            }
            None => String::from("Unknown"),
        };

        let time_formatted = {
            format!(
                "{} - {}",
                to_time(track_info.position.as_secs()),
                to_time(metadata.duration.unwrap().as_secs())
            )
        };

        ctx.channel_id().send_message(&ctx, |m| {
            m.embed(|e| e
                .colour(0xffffff)
                .title(metadata.title.clone().unwrap_or_else(|| String::from("Unknown")))
                .thumbnail(metadata.thumbnail.clone().unwrap_or_else(|| String::from("https://images.unsplash.com/photo-1611162616475-46b635cb6868?ixlib=rb-4.0.3")))
                .url(metadata.source_url.clone().unwrap())
                .fields(vec![
                    ("Artist", metadata.artist.clone().unwrap_or_else(|| String::from("Unknown")), false),
                    ("Released", date_formatted, true),
                    ("Position", time_formatted, true),
                    ("Status", format!("{:?}", track_info.playing), true),
                ])
                .timestamp(Timestamp::now())
            )
        }).await?;
         
    } else {
        let reply = CreateReply::default()
            .content(format!("You need to be in a voice channel! {}", random_emote(Mood::Negative)))
            .ephemeral(true);
        ctx.send(reply).await?;
    }
     */
    Ok(())
}
