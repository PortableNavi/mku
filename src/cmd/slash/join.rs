use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;


use serenity::utils::{Color, MessageBuilder};

use mku::join_channel;


pub async fn run<'a>(
    embed: &'a mut CreateEmbed,
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> &'a mut CreateEmbed
{
    let color;
    let mut response = MessageBuilder::new();
    let msg = interaction.to_owned();

    response.push(
        match join_channel(ctx, &msg.guild_id.unwrap(), &msg.user.id, &msg.channel_id).await
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
    );

    response.build();
    embed.description(&response).color(color)
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("join")
        .description("Invite me to join your voicechat")
}
