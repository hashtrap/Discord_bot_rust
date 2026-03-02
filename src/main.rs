use poise::serenity_prelude as serenity;
use serenity::model::id::{GuildId,CommandId};
use dotenv;
use serenity::all::ApplicationId;

struct Data{}
type Error=Box<dyn std::error::Error+Send+Sync>;
type Context<'a>=poise::Context<'a,Data,Error>;

//Below are the extra functions that are used because i like to

fn prepare_env()
{

    dotenv::from_filename(".env").ok();
    println!("Preparing environment variables...");

}

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

    let response="Hello im a simple bot, serving as this motels bell hop";
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn duet(ctx:Context<'_>)->Result<(),Error>
{

    let response="Duet works";
    ctx.say(response).await?;
    Ok(())
}


#[tokio::main]
async fn main()
{
    prepare_env();

    let token= std::env::var("DISCORD_TOKEN").expect("You forgot the fucking token you moron");

    let app_id= std::env::var("APPLICATION_ID").expect("You forgot the fucking app_id you moron");

    let intents=serenity::GatewayIntents::non_privileged();

    let framework= poise::Framework::builder()
        .options(poise::FrameworkOptions
        {
            commands:vec![duet(),hello()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework|
            {
            Box::pin(async move
                {
                poise::builtins::register_in_guild(ctx, &framework.options().commands,GuildId::new(1470487901958574214)).await?;
                Ok(Data {})
                })
            })
        .build();

    let client=serenity::ClientBuilder::new(token, intents)
    .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}