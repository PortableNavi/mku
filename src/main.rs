mod cmd;

use std::collections::HashSet;
use std::sync::Arc;

use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use serenity::utils::Color;
use songbird::SerenityInit;

use tracing::{error, info};

use cmd::join::*;
use cmd::play::*;
use cmd::sing::*;


pub struct MkuShardManagerContainer;

impl TypeMapKey for MkuShardManagerContainer
{
    type Value = Arc<Mutex<ShardManager>>;
}


struct MkuHandler;

#[async_trait]
impl EventHandler for MkuHandler
{
    async fn ready(&self, ctx: Context, ready: Ready)
    {
        info!("{} is online", ready.user.name);

        let guild_id = GuildId(
            std::env::var("MKU_GUILD_ID")
                .expect("MKU_GUILD_ID is not set")
                .parse()
                .expect("MKU_GUILD_ID must be an integer"),
        );

        guild_id
            .set_application_commands(&ctx.http, |commands| {
                commands.create_application_command(|command| cmd::slash::sing::register(command))
            })
            .await
            .expect("Failed to register slash commands");
    }

    async fn resume(&self, _ctx: Context, _: ResumedEvent)
    {
        info!("Resumed");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction)
    {
        let mut create_embed = CreateEmbed::default();

        if let Interaction::ApplicationCommand(command) = interaction
        {
            let embed = match command.data.name.as_str()
            {
                "sing" => cmd::slash::sing::run(&command.data.options, &mut create_embed),

                _ => create_embed
                    .title("Ups...")
                    .description(
                        "I don't know that slash command... How did you even sent that to me?",
                    )
                    .color(Color::RED),
            };

            if let Err(e) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| msg.add_embed(embed.to_owned()))
                })
                .await
            {
                println!("Fuck: {e:?}");
            }
        }
    }
}


#[group]
#[commands(sing, play, join)]
struct Music;


#[tokio::main]
async fn main()
{
    let token = std::env::var("MKU_BOT_TOKEN").expect("MKU_BOT_TOKEN is not set");
    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await
    {
        Ok(info) =>
        {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }

        Err(e) => panic!("Failed to fetch application info: {e:?}"),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("mk!"))
        .group(&MUSIC_GROUP);

    let intends = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&token, intends)
        .framework(framework)
        .event_handler(MkuHandler)
        .register_songbird()
        .await
        .expect("Failed to build client");

    {
        let mut data = client.data.write().await;
        data.insert::<MkuShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to register ctrl-c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(e) = client.start().await
    {
        error!("Client Error: {:?}", e);
    }
}
