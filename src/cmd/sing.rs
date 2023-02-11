use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Color, MessageBuilder};


#[command]
async fn sing(ctx: &Context, msg: &Message, args: Args) -> CommandResult
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
