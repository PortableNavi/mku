use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::utils::{Color, MessageBuilder};


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
