use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use serenity::utils::{Color, MessageBuilder};


pub async fn run<'a>(
    options: &'a [CommandDataOption],
    embed: &'a mut CreateEmbed,
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> &'a mut CreateEmbed
{
    let color;
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

    response.push(
        match mku::join_channel(ctx, &msg.guild_id.unwrap(), &msg.user.id, &msg.channel_id).await
        {
            Ok(_) => match mku::queue_song(ctx, &msg.guild_id.unwrap(), &url).await
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
