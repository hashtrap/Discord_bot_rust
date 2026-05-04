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

    pub fn prepare(&self,access_token:String,token_type: String, expires_in: u32)->Spotify_Token
    {
        Spotify_Token{access_token,token_type,expires_in}
    }

}

async fn connect_client(client:reqwest::Client, spotify_token: &mut Spotify_Token)->Result<(),reqwest::Error>
{
    let client_id = get_env_var("CLIENT_ID");
    let client_secret = get_env_var("CLIENT_SECRET");

    let response=client.post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("grant_type=client_credentials&client_id={}&client_secret={}",client_id,client_secret))
        .send().await?;


    let spotify_token= response.json::<Spotify_Token>().await?;

    println!("Token is: {}, bearer: {}, and ttl is: {}",spotify_token.access_token,spotify_token.token_type,spotify_token.expires_in);

    Ok(())
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
}


