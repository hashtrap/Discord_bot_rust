use std::io::ErrorKind::FileTooLarge;
use reqwest::Client;
use crate::api::*;
use crate::get_env_var;

#[derive(Serialize,Deserialize)]
struct Spotify_Token
{
    access_token: String,
    token_type: String,
    expires_in: u32,
}

impl Spotify_Token
{
    pub fn new() -> Spotify_Token
    {
        Spotify_Token{ access_token:String::from(""), token_type:String::from(""), expires_in:0 }
    }

    pub fn prepare(&self,access_token:&str,token_type: &str, expires_in: u32)->Spotify_Token
    {
        Spotify_Token{access_token:access_token.into(),token_type:token_type.into(),expires_in:expires_in.into()}
    }

}

async fn connect_client(client:reqwest::Client, spotify_token: &mut Spotify_Token) ->Result<(),reqwest::Error>
{
    let client_id = get_env_var("CLIENT_ID");
    let client_secret = get_env_var("CLIENT_SECRET");

    let response=client.post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("grant_type=client_credentials&client_id={}&client_secret={}",client_id,client_secret))
        .send().await?;


    *spotify_token= response.json::<Spotify_Token>().await?;

    println!("Token is: {}, bearer: {}, and ttl is: {}",spotify_token.access_token,spotify_token.token_type,spotify_token.expires_in);

    Ok(())
}

async fn get_playlist(spotify_token:&Spotify_Token,client: Client)->Result<(),reqwest::Error>
{
    let playlist_id = get_env_var("PLAYLIST_ID");



    todo!()
}

#[cfg(test)]
mod tests
{
    use crate::prepare_env;
    use super::*;

    #[tokio::test]
    async fn test_connection()
    {

        prepare_env();

    }


    #[tokio::test]
    async fn test_playlist_retrieval()
    {

        prepare_env();

    }
}


