use std::env;
use std::string::String;

pub mod api;
mod Similarity;

pub  fn get_env_var(var_name:&str) ->String
{
    env::var(var_name).expect("Variable has not been set")
}
pub  fn prepare_env()

{

    dotenv::from_filename(".env").ok();
    println!("Preparing environment variables...");

}





