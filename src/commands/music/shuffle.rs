use poise::CreateReply;
use rand::Rng;
use crate::Error;
use crate::util::globals::{Mood, random_emote};

#[poise::command(slash_command, prefix_command)]
pub async fn shuffle(
    ctx: crate::Context<'_>,
) -> Result<(), Error> {

    let guild_id = ctx.guild().unwrap().id;

    let manager = songbird::get(ctx.as_ref())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        queue.modify_queue(|queue| {
            // skip the first track on queue because it's being played
            fisher_yates_shuffle(
                queue.make_contiguous()[1..].as_mut(),
                &mut rand::thread_rng(),
            )
        });

        let reply = CreateReply::default()
            .content(format!(":game_die: Queue shuffled! {}", random_emote(Mood::Neutral)))
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

fn fisher_yates_shuffle<T, R>(arr: &mut [T], mut rng: R)
where
    R: rand::RngCore + Sized,
{
    let mut index = arr.len();
    while index >= 2 {
        index -= 1;
        arr.swap(index, rng.gen_range(0..(index + 1)));
    }
}
