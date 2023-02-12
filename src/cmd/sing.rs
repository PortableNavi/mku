use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Color, MessageBuilder};


#[command]
pub async fn sing(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{
    let mut reponse = MessageBuilder::new();

    for arg in args.raw()
    {
        reponse.push_italic_safe(arg);
        reponse.push(" ");
    }

    reponse.build();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.description(&reponse)
                    .color(Color::from_rgb(244, 219, 214))
            })
        })
        .await?;

    Ok(())
}


pub fn run<'a>(options: &'a [CommandDataOption], embed: &'a mut CreateEmbed)
    -> &'a mut CreateEmbed
{
    let text = options
        .first()
        .expect("Expected a string option")
        .resolved
        .as_ref()
        .expect("Expected a string object");

    let reponse;

    if let CommandDataOptionValue::String(t) = text
    {
        reponse = MessageBuilder::new().push_italic_safe(t).build();
    }
    else
    {
        reponse = MessageBuilder::new()
            .push_italic_safe("Oh no... i cant sing that.")
            .build();
    };

    embed.description(&reponse).color(Color::ROSEWATER)
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("sing")
        .description("I will sing anything you say")
        .create_option(|option| {
            option
                .name("text")
                .description("This is what i am gonna sing")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
