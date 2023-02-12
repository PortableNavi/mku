use std::sync::Arc;


use serenity::async_trait;

use serenity::http::Http;

use serenity::model::channel::Message;
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::prelude::UserId;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use songbird::{
    input::Restartable, Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent,
};

use songbird::input::Input;


struct TrackEndNotifier
{
    chan_id: ChannelId,
    http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier
{
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event>
    {
        if let EventContext::Track(_track_list) = ctx
        {
            /*
            self.chan_id
                .say(&self.http, &format!("Tracks ended: {}.", track_list.len()))
                .await
                .unwrap();
            */
        }

        None
    }
}


pub async fn join_channel(
    ctx: &Context,
    guild_id: &GuildId,
    author_id: &UserId,
    channel_id: &ChannelId,
) -> Result<String, String>
{
    let guild = ctx.cache.guild(guild_id).unwrap();

    let voice_id = guild.voice_states.get(author_id).and_then(|v| v.channel_id);

    let connection = match voice_id
    {
        Some(channel) => channel,
        None =>
        {
            return Err("I cannot join you, since you aren't connected to any voice channel right now. Please join a voice channel and try again.".to_string());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird must be initialized first")
        .clone();

    let (hlock, success) = manager.join(guild_id.to_owned(), connection).await;

    return if let Ok(_) = success
    {
        let chan_id = channel_id.to_owned();
        let http = ctx.http.clone();
        let mut handle = hlock.lock().await;

        handle.add_global_event(
            Event::Track(TrackEvent::End),
            TrackEndNotifier { chan_id, http },
        );

        Ok(format!("**Joined:** {}", connection.mention()))
    }
    else
    {
        Err("Something went wrong. I can't join your channel right now...".to_string())
    };
}


pub async fn queue_song(ctx: &Context, guild_id: &GuildId, url: &str) -> Result<String, String>
{
    if !url.starts_with("http")
    {
        return Err(
            "Oh no, your URL does not seem to be valid. I dont know what to play...".to_string(),
        );
    }

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird must be initialized first.")
        .clone();

    if let Some(hlock) = manager.get(guild_id.to_owned())
    {
        let mut handler = hlock.lock().await;

        let src = match Restartable::ytdl(url.to_owned(), true).await
        {
            Ok(source) => source,
            Err(_) =>
            {
                return Err("Oh no, i couldnt find any music to play at that link...".to_string());
            }
        };

        handler.enqueue_source(Input::from(src));

        let mut title = "Song".to_string();
        let mut duration = "".to_string();

        if let Some(last) = &handler.queue().current_queue().last()
        {
            title = last.metadata().clone().title.unwrap_or("Song".to_string());
            duration = "(3:09)".to_string()
        }

        return Ok(format!(
            "**Queued** *{:?}* **at position:** *{:?}*",
            format!("{title} {duration}"),
            handler.queue().len()
        ));
    }

    Ok("".to_string())
}

pub async fn view_queue(ctx: &Context, msg: &Message) -> Result<String, String>
{
    let guild = msg.guild(&ctx.cache).unwrap();
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird must be initialized first")
        .clone();

    return if let Some(hlock) = manager.get(guild.id)
    {
        let handle = hlock.lock().await;
        let mut response = MessageBuilder::new();

        let mut pos = handle.queue().current_queue().len();

        if pos < 1
        {
            return Ok("Nothing is queued right now".to_string());
        }

        for song in handle.queue().current_queue()
        {
            let title = song.metadata().clone().title;
            response.push_bold(format!("[{pos}] "));
            response.push_line_safe(title.unwrap_or("Song".to_string()));
            pos -= 1;
        }

        Ok(response.to_string())
    }
    else
    {
        Err("Unable to fetch queue".to_string())
    };
}
