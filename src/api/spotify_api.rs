use crate::api::*;
use crate::get_env_var;

#[derive(Serialize,Deserialize)]
struct Spotify_Token
{
    access_token: String,
    token_type: String,
    expires_in: i32,
}

struct Client
{
    client:reqwest::Client,
    client_id:String,
    client_secret:String,
}

impl Client
{
    pub fn new(target:reqwest::Client)->Client
    {
        Client
        {
            client:target,
            client_id:get_env_var("CLIENT_ID"),
            client_secret:get_env_var("CLIENT_SECRET"),
        }
    }
}


impl Spotify_Token
{
    pub fn new() -> Spotify_Token
    {
        Spotify_Token{ access_token:String::from(""), token_type:String::from(""), expires_in:0 }
    }

}

async fn connect_client(client:Client, spotify_token: &mut Spotify_Token)->Result<String,reqwest::Error>
{
    let response=client.client.post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("grant_type=client_credentials&client_id={}&client_secret={}",client.client_id,client.client_secret))
        .send().await?;


    let spotify_token= response.json::<Spotify_Token>().await?;

    println!("Token is: {}, bearer: {}, and ttl is: {}",spotify_token.access_token,spotify_token.token_type,spotify_token.expires_in);

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

        let a=reqwest::Client::new();
        let client=Client::new(a);

        let mut spotify_token=Spotify_Token::new();

        let finalist=match  connect_client(client, &mut  spotify_token).await
        {
            Ok(result) => println!("{}",result),
            Err(_) =>println!("Something went wrong"),
        };



    }
}


