use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::{Color, MessageBuilder};

use mku::join_channel;

#[command]
async fn join(ctx: &Context, msg: &Message) -> CommandResult
{
    let color;
    let mut reponse = MessageBuilder::new();

    let reponse_txt =
        match join_channel(ctx, &msg.guild_id.unwrap(), &msg.author.id, &msg.channel_id).await
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
