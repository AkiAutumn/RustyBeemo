use poise::CreateReply;
use serenity::model::prelude::*;
use crate::Error;
use crate::util::globals::{Mood, random_emote};

#[poise::command(slash_command, prefix_command)]
pub async fn queue(
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

        let _ = match queue.current() {
            Some(current) => current,
            None => {
                let reply = CreateReply::default()
                    .content("Nothing is playing right now!")
                    .ephemeral(true);
                ctx.send(reply).await?;

                return Ok(());
            }
        };

        let mut desc = String::from("+ - + - + - + - + - + - + - + - + - +\n");

        let mut total_time = 0;
        for (i, song) in queue.current_queue().iter().enumerate() {
            desc.push_str(&format!(
                "{}. {} - {}\n",
                i + 1,
                song.metadata().title.clone().unwrap(),
                song.metadata()
                    .artist
                    .clone()
                    .unwrap_or_else(|| String::from("Unknown"))
            ));
            total_time += song.metadata().duration.unwrap().as_secs()
        }

        ctx.channel_id()
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.colour(0xffffff)
                        .title(":notes: - Queue - :notes:")
                        .fields(vec![
                            ("Queue length", format!("{}", queue.len()), true),
                            //("Total time", to_time(total_time), true),
                        ])
                        .description(desc)
                        .timestamp(Timestamp::now())
                })
            })
            .await?;

    } else {
        
        let reply = CreateReply::default()
            .content(format!("You need to be in a voice channel! {}", random_emote(Mood::Negative)))
            .ephemeral(true);
        ctx.send(reply).await?;
    }
     */
    Ok(())
}
