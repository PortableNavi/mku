use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Color, MessageBuilder};

use mku::queue_song;


async fn body(
    ctx: &Context,
    msg: &mut MessageBuilder,
    guild_id: &GuildId,
    user_id: &UserId,
    channel_id: &ChannelId,
    url: &str,
) -> Color
{
    let color;

    msg.push(
        match mku::join_channel(ctx, guild_id, user_id, channel_id).await
        {
            Ok(_) => match queue_song(ctx, guild_id, url).await
            {
                Ok(resp) =>
                {
                    color = Color::ROSEWATER;
                    resp
                }

                Err(resp) =>
                {
                    color = Color::RED;
                    resp
                }
            },

            Err(resp) =>
            {
                color = Color::RED;
                resp
            }
        },
    );

    color
}


#[command]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
    let mut response = MessageBuilder::new();
    let url: String = args.single()?;

    let color = body(
        ctx,
        &mut response,
        &msg.guild_id.unwrap(),
        &msg.author.id,
        &msg.channel_id,
        &url,
    )
    .await;

    response.build();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.description(&response).color(color)
                //.image("https://pm1.narvii.com/6723/20abee64db783227e8096ae91660987cb27c215b_hq.jpg")
            })
        })
        .await?;

    Ok(())
}


pub async fn run<'a>(
    options: &'a [CommandDataOption],
    embed: &'a mut CreateEmbed,
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> &'a mut CreateEmbed
{
    let mut response = MessageBuilder::new();
    let msg = interaction.to_owned();

    let url_wrapped = options
        .first()
        .expect("Expected a string option")
        .resolved
        .as_ref()
        .expect("Expected a string object");

    let mut url = "".to_string();

    if let CommandDataOptionValue::String(t) = url_wrapped
    {
        url = MessageBuilder::new().push(t).build();
    }

    let color = body(
        ctx,
        &mut response,
        &msg.guild_id.unwrap(),
        &msg.user.id,
        &msg.channel_id,
        &url,
    )
    .await;

    response.build();
    embed.description(&response).color(color)
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("play")
        .description("I will add a song from a URL to the current queue")
        .create_option(|option| {
            option
                .name("url")
                .description("Im going to look for something to play at that URL")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
