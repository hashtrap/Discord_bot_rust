use dotenv;
use std::string::String;


pub mod api;
pub mod similarity;
mod menu_helpter;

pub  fn get_env_var(var_name:&str) ->String
{
    dotenv::var(var_name).expect(&format!("Variable {:?} has not been set",var_name))
}
pub  fn prepare_env()

{

    dotenv::from_filename("./.env").ok();
    println!("Preparing environment variables...");

}





