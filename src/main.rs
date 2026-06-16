use std::sync::Arc;
use std::time::Duration;
use discord_bot::{prepare_env, get_env_var};
use poise::serenity_prelude as serenity;
use serenity::model::prelude::*;
use serenity::all::{ApplicationId, EventHandler, Message};
use serenity::async_trait;
use serenity::builder::Builder;
use discord_bot::api::open_subsonic_api;

struct Handler;
struct Data
{
    client: reqwest::Client,
}
type Error=Box<dyn std::error::Error+Send+Sync>;
type Context<'a>=poise::Context<'a,Data,Error>;

#[async_trait]
impl EventHandler for Handler
{
        async fn cache_ready(&self, ctx2: serenity::Context,_guilds:Vec<GuildId>)
        {
            let http=Arc::clone(&ctx2.http);
            let channel_id=ChannelId::new(1480926914461040811);
            let guild_id=Some(GuildId::new(1470487901958574214));

            tokio::spawn(async move
                {
                    let mut interval=tokio::time::interval(Duration::from_hours(24));

                    loop
                    {
                        interval.tick().await;
                        let message=serenity::builder::CreateMessage::new();
                        let menu=menu_creator();


                        let message=message.add_embed(menu);

                        let _message=message.execute(&http,(channel_id,guild_id)).await;

                    }

                });


        }
}

fn menu_creator()->serenity::builder::CreateEmbed
{

    let menu=serenity::builder::CreateEmbed::new();



    let menu=menu.color(Color::from_rgb(255, 49, 49));

    let menu=menu.description("Start a duet today with a random or a song of your choice 🎵");


    menu

}


//Below are located all the disocrd/poise framework related functions


#[poise::command(slash_command)]

async fn hello(ctx:Context<'_>) -> Result<(),Error>
{

    let response="Hello to you to. Welcome to the Hazbin Motel  UwU";
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]

async fn duet_random(ctx:Context<'_>)->Result<(),Error>
{
    let mut interval=tokio::time::interval(Duration::from_secs(3));
    ctx.defer_ephemeral().await?;
    let lyrics: Result<String, ()> = Err(());
    match lyrics
    {
        Ok(lyrics) =>
            {
                println!("Point reached");
                lyrics
            },
        Err(_) =>
            {
                println!("Error reached");
                ctx.say("Oops something went wrong while getting the song, please try again later QwQ").await?;
                return Ok(());
            }
    };
    for line in lyrics.iter()
    {
        interval.tick().await;
        ctx.say(line).await?;
    }

    ctx.say("Duet Done  OwO").await?;



    Ok(())
}

#[poise::command(slash_command)]
async fn duet_song(ctx:Context<'_>)->Result<(),Error>
{


    Ok(())
}


#[tokio::main]
async fn main()
{
    prepare_env();



    let client =reqwest::Client::new();



    let token= get_env_var("DISCORD_TOKEN");

    //let app_id= get_env_var("APPLICATION_ID");

    let guild_id= get_env_var("GUILD_ID");



            let intents=serenity::GatewayIntents::non_privileged();

            let framework= poise::Framework::builder()
                .options(poise::FrameworkOptions
                {
                    commands:vec![ hello(),duet_song(),duet_random()],
                    ..Default::default()
                })
                .setup(|ctx, _ready, framework|
                    {
                        Box::pin(async move
                            {
                                poise::builtins::register_in_guild(ctx, &framework.options().commands,GuildId::new(guild_id.parse().unwrap())).await?;
                                Ok(Data { client: client })
                            })
                    })
                .build();

            let client=serenity::ClientBuilder::new(token, intents)
                .event_handler(Handler)
                .framework(framework)
                .await;
            client.unwrap().start().await.unwrap();




}

