use discord_bot::{prepare_env, create_client};
use poise::serenity_prelude as serenity;
use serenity::model::id::{GuildId,CommandId};
use serenity::all::ApplicationId;

struct Data
{
    spotify_client: reqwest::Client,
}
type Error=Box<dyn std::error::Error+Send+Sync>;
type Context<'a>=poise::Context<'a,Data,Error>;

//Below are the extra functions that are used because i like to



//Use only for testing or to delete a global command created by mistake
async fn clear_global_command(token:String,app_id:String)
{
    let id:u64=app_id.parse::<u64>().expect("You forgot to the integer");
    let http=serenity::http::Http::new(&token);
    http.set_application_id(ApplicationId::new(id));


    let result=http.delete_global_command(CommandId::new(1472322847161585765)).await;


    if result.is_ok()
    {
        println!("command erasure done");
    }

    else
    {
        println!("command erasure not done {}",result.err().unwrap());
    }
}

//Below are located all the disocrd/poise framework related functions

#[poise::command(slash_command)]

async fn hello(ctx:Context<'_>) -> Result<(),Error>
{

    let response="Client succesfully passed";
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]

async fn duet2(ctx:Context<'_>)->Result<(),Error>
{


    Ok(())
}

#[poise::command(slash_command)]

async fn ping(ctx:Context<'_>)->Result<(),Error>
{


    Ok(())
}


#[tokio::main]
async fn main()
{
    prepare_env();

    let Spotify_Client=create_client();
    let spotify_client = Spotify_Client.clone();



    tokio::spawn(async move
        {
            let token= std::env::var("DISCORD_TOKEN").expect("You forgot the fucking token you moron");

            let app_id= std::env::var("APPLICATION_ID").expect("You forgot the fucking app_id you moron");

            let intents=serenity::GatewayIntents::non_privileged();

            let framework= poise::Framework::builder()
                .options(poise::FrameworkOptions
                {
                    commands:vec![duet2(), hello(),ping()],
                    ..Default::default()
                })
                .setup(|ctx, _ready, framework|
                    {
                        Box::pin(async move
                            {
                                poise::builtins::register_in_guild(ctx, &framework.options().commands,GuildId::new(1470487901958574214)).await?;
                                Ok(Data {spotify_client:spotify_client})
                            })
                    })
                .build();

            let client=serenity::ClientBuilder::new(token, intents)
                .framework(framework)
                .await;
            client.unwrap().start().await.unwrap();
        });

    tokio::spawn(async move
        {

        });
}

