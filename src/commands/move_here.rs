use crate::{Context};
use crate::Error;
use poise::serenity_prelude as serenity;
use serenity::all::{ChannelId, ChannelType, GuildChannel};
use crate::util::globals::{Mood, random_emote};

/// Moves all user's from one voice channel to another
#[poise::command(slash_command, prefix_command)]
pub async fn move_here(
    ctx: Context<'_>,
    #[description = "Select the target voice-channel"] c: Option<GuildChannel>,
) -> Result<(), Error> {
    /*
    let destination_channel = c.unwrap();
    let author = ctx.author();
    let guild = ctx.guild().unwrap();
    let author_voice_channel_id = match guild.voice_states.get(&author.id).unwrap().channel_id {
        Some(channel_id) => channel_id,
        None => {
            ctx.reply("You need to be in a voice channel you want to move members from.").await?;
            return Ok(());
        }
    };
    
    if destination_channel.kind != ChannelType::Voice {
        ctx.reply("Please provide a valid voice channel as an argument.").await?;
        return Ok(());
    }
    
    let source_voice_channel = guild.channels.get(&author_voice_channel_id).unwrap();
    let members = source_voice_channel.members(ctx).unwrap();

    for member in members {
        member.move_to_voice_channel(ctx, destination_channel.id).await?;
    }

    let response = format!("Moved everyone! {}", random_emote(Mood::Positive));
    ctx.reply(response).await?;
    
     */
    ctx.reply("Feature under development...").await?;
    Ok(())
}