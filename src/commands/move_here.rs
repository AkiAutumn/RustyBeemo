use crate::{Context};
use crate::Error;
use poise::{CreateReply, serenity_prelude as serenity};
use serenity::all::{ChannelId, ChannelType, GuildChannel};
use crate::util::globals::{Mood, random_emote};

/// Moves all user's from your current voice channel to another
#[poise::command(slash_command, prefix_command)]
pub async fn move_here(
    ctx: Context<'_>,
    #[description = "Select the target voice-channel"] voice_channel: Option<GuildChannel>,
) -> Result<(), Error> {

    let destination_channel = voice_channel.unwrap();
    let author = ctx.author();
    let guild = ctx.guild().unwrap().clone();

    let author_voice_state = match guild.voice_states.get(&author.id) {
        Some(vs) => vs,
        None => {
            
            let reply = CreateReply::default()
                .content("You need to be in a voice channel you want to move members from.")
                .ephemeral(true);
            ctx.send(reply).await?;
            
            return Ok(());
        }
    };
    
    if destination_channel.kind != ChannelType::Voice {
        
        let reply = CreateReply::default()
            .content("Please provide a valid voice channel as an argument.")
            .ephemeral(true);
        ctx.send(reply).await?;
        
        return Ok(());
    }
    
    let channel_id = &author_voice_state.channel_id.unwrap();
    let source_voice_channel = guild.channels.get(channel_id).unwrap();
    let members = source_voice_channel.members(ctx).unwrap();

    for member in members {
        member.move_to_voice_channel(ctx, destination_channel.id).await?;
    }
    
    let reply = CreateReply::default()
        .content(format!("Moved everyone! {}", random_emote(Mood::Positive)))
        .ephemeral(true);
    ctx.send(reply).await?;

    Ok(())
}