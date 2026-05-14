use std::env;
use reqwest;
use std::string::String;
use dotenv::dotenv;

pub mod api;

pub  fn get_env_var(var_name:&str)->String
{
    env::var(var_name).expect("Variable has not been set")
}
pub  fn prepare_env()

{

    dotenv::from_filename(".env").ok();
    println!("Preparing environment variables...");

}

pub  fn create_client()->reqwest::Client
{
    reqwest::Client::new()
}



