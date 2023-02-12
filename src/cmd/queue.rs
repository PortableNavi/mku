use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Color, MessageBuilder};

use mku::view_queue;

#[command]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult
{
    let color;
    let mut reponse = MessageBuilder::new();

    let reponse_txt = match view_queue(ctx, msg).await
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
    };

    reponse.push(reponse_txt);
    reponse.build();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| e.description(&reponse).color(color))
        })
        .await?;

    Ok(())
}
