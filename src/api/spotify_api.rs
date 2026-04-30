use crate::api::*;
use crate::get_env_var;

#[derive(Serialize,Deserialize)]
struct Spotify_Token
{
    access_token: String,
    token_type: String,
    expires_in: i32,
}


impl Spotify_Token
{
    /*
    pub fn init(&self) -> Spotify_Token
    {


    }

     */
}

