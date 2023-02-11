use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Color, MessageBuilder};

use mku;

#[command]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult
{
    let color;
    let mut reponse = MessageBuilder::new();
    let url: String = args.single()?;

    let join_response: String = match mku::join_channel(ctx, msg).await
    {
        Ok(_) => match mku::queue_song(ctx, msg, &url).await
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
    };

    reponse.push(join_response);
    reponse.build();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.description(&reponse).color(color)
                //.image("https://pm1.narvii.com/6723/20abee64db783227e8096ae91660987cb27c215b_hq.jpg")
            })
        })
        .await?;

    Ok(())
}
