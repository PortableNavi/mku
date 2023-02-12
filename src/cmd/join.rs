use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Color, MessageBuilder};

use mku::join_channel;


async fn body(
    ctx: &Context,
    msg: &mut MessageBuilder,
    guild_id: &GuildId,
    user_id: &UserId,
    channel_id: &ChannelId,
) -> Color
{
    let color;

    msg.push(match join_channel(ctx, guild_id, user_id, channel_id).await
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
    });

    color
}


#[command]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult
{
    let mut response = MessageBuilder::new();

    let color = body(
        ctx,
        &mut response,
        &msg.guild_id.unwrap(),
        &msg.author.id,
        &msg.channel_id,
    )
    .await;

    response.build();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| e.description(&response).color(color))
        })
        .await?;

    Ok(())
}


pub async fn run<'a>(
    embed: &'a mut CreateEmbed,
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> &'a mut CreateEmbed
{
    let mut response = MessageBuilder::new();
    let msg = interaction.to_owned();

    let col = body(
        ctx,
        &mut response,
        &msg.guild_id.unwrap(),
        &msg.user.id,
        &msg.channel_id,
    )
    .await;

    response.build();
    embed.description(&response).color(col)
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("join")
        .description("Invite me to join your voice chat")
}
