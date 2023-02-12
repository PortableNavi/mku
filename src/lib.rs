use std::sync::Arc;


use serenity::async_trait;

use serenity::http::Http;


use serenity::model::id::{ChannelId, GuildId};
use serenity::model::prelude::UserId;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use songbird::{
    input::Restartable, Event, EventContext, EventHandler as VoiceEventHandler, Songbird,
    TrackEvent,
};

use songbird::input::Input;


struct TrackEndNotifier
{
    manager: Arc<Songbird>,
    guild_id: GuildId,
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier
{
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event>
    {
        if let EventContext::Track(track_list) = ctx
        {
            if track_list.len() < 2
            {
                let _ = self.manager.leave(self.guild_id).await;
            }
        }

        None
    }
}


struct TrackStartNotifier
{
    chan_id: ChannelId,
    http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for TrackStartNotifier
{
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event>
    {
        if let EventContext::Track(tracklist) = ctx
        {
            let mut response = MessageBuilder::new();
            response.push_bold("Now Playing: ");

            response.push_italic_safe(match tracklist.first()
            {
                Some((_, handle)) => handle
                    .metadata()
                    .to_owned()
                    .title
                    .unwrap_or("Song".to_string()),
                None => "Song".to_string(),
            });

            let _ = self
                .chan_id
                .send_message(&self.http, |msg| {
                    msg.add_embed(|e| e.description(&response))
                })
                .await;
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

        handle.remove_all_global_events();
        handle.add_global_event(
            Event::Track(TrackEvent::Play),
            TrackStartNotifier { chan_id, http },
        );

        handle.add_global_event(
            Event::Track(TrackEvent::End),
            TrackEndNotifier {
                manager,
                guild_id: guild_id.to_owned(),
            },
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
            duration = "".to_string() // TODO: replace with actual duration
        }

        return Ok(format!(
            "**Queued** *{:?}* **at position:** *{:?}*",
            format!("{title} {duration}"),
            handler.queue().len()
        ));
    }

    Ok("".to_string())
}

pub async fn view_queue(ctx: &Context, guild_id: &GuildId) -> Result<String, String>
{
    let _guild = ctx.cache.guild(guild_id);

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird must be initialized first")
        .clone();

    return if let Some(hlock) = manager.get(guild_id.to_owned())
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


pub async fn skip(ctx: &Context, guild_id: &GuildId) -> Result<String, String>
{
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird must be initialized first")
        .clone();

    return if let Some(hlock) = manager.get(guild_id.to_owned())
    {
        let handle = hlock.lock().await;

        let title;

        if let Some(track) = handle.queue().current()
        {
            title = track
                .metadata()
                .to_owned()
                .title
                .unwrap_or("Song".to_string());
        }
        else
        {
            title = "Song".to_string();
        }

        match handle.queue().skip()
        {
            Err(_) => Err("I tried my best but i wasn't able to skip that song".to_string()),
            Ok(_) => Ok(format!("**Ok, skipped:** *{title:?}*")),
        }
    }
    else
    {
        return Err("I cant skip songs for you if we're not in the same voice chat. You can use mk!join to move me into your voice chat".to_string());
    };
}
