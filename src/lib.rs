use std::env;

pub mod api;

pub  fn get_env_var(var_error:&str)->String
{
    env::var(var_error).expect("Variable has not been set")
}
pub  fn prepare_env()
{

    dotenv::from_filename(".env").ok();
    println!("Preparing environment variables...");

}



