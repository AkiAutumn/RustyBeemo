use poise::CreateReply;
use songbird::input::YoutubeDl;

use crate::{Data, Error};
use crate::util::globals::{Mood, random_emote};

struct HttpKey;
#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: crate::Context<'_>,
    #[description = "URL or song name"] search: Option<String>,
) -> Result<(), Error> {

    let url = search.unwrap();

    let do_search = !url.starts_with("http");

    let guild_id = ctx.guild_id().unwrap();

    let http_client = reqwest::Client::new();

    let manager = songbird::get(ctx.as_ref())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let mut src = if do_search {
            YoutubeDl::new_search(http_client, url)
        } else {
            YoutubeDl::new(http_client, url)
        };
        let _ = handler.play_input(src.clone().into());

        let reply = CreateReply::default()
            .content(format!("Playing... :notes: {}", random_emote(Mood::Positive)))
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
