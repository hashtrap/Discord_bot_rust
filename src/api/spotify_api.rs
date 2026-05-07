#![allow(warnings)]

use std::string::String;
use crate::api::*;
use crate::get_env_var;

#[derive(Deserialize, Debug)]
struct Playlist {
    name: String,
    tracks: Tracks,
}

#[derive(Deserialize, Debug)]
struct Tracks {
    items: Vec<TrackItem>,
}

#[derive(Deserialize, Debug)]
struct TrackItem {
    track: Track,
}

#[derive(Deserialize, Debug)]
struct Track {
    name: String,
}

#[derive(Serialize,Deserialize)]
struct SpotifyToken
{
    access_token: String,
    token_type: String,
    expires_in: u32,
}

impl SpotifyToken
{
    pub fn new() -> SpotifyToken
    {
        SpotifyToken { access_token:String::from(""), token_type:String::from(""), expires_in:0 }
    }

    pub fn prepare(&self,access_token:&str,token_type: &str, expires_in: u32)-> SpotifyToken
    {
        SpotifyToken {access_token:access_token.into(),token_type:token_type.into(),expires_in:expires_in.into()}
    }

}

async fn connect_client(client:reqwest::Client, spotify_token: &mut SpotifyToken) ->Result<(),reqwest::Error>
{
    let client_id = get_env_var("CLIENT_ID");
    let client_secret = get_env_var("CLIENT_SECRET");

    let response=client.post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("grant_type=client_credentials&client_id={}&client_secret={}",client_id,client_secret))
        .send().await?;


    *spotify_token= response.json::<SpotifyToken>().await?;

    //println!("Token is: {}, bearer: {}, and ttl is: {}",spotify_token.access_token,spotify_token.token_type,spotify_token.expires_in);

    Ok(())
}

async fn get_playlist(spotify_token:&SpotifyToken, client: reqwest::Client) ->Result<(),reqwest::Error>
{
    let playlist_id = get_env_var("PLAYLIST_ID");

    let response=client.get(format!("https://api.spotify.com/v1/playlists/{}?fields=name,tracks.items(track(name))",playlist_id))
        .header("Authorization",format!("Bearer {}", spotify_token.access_token))
        .send().await?;

    println!("response when getting playlist: \n {:?} ", response);

    //let playlist:Playlist = response.json::<Playlist>().await?;

    todo!()
}

#[cfg(test)]
mod tests
{
    #![allow(warnings)]
    use crate::prepare_env;
    use super::*;

    #[tokio::test]
    async fn test_connection()
    {

        prepare_env();

        let client=reqwest::Client::new();
        let mut token  = SpotifyToken::new();

        connect_client(client.clone(),&mut token).await.unwrap();



    }


    #[tokio::test]
    async fn test_playlist_retrieval()
    {

        prepare_env();

        let client=reqwest::Client::new();
        let mut token  = SpotifyToken::new();

        connect_client(client.clone(),&mut token).await.unwrap();
        get_playlist(&token,client).await.unwrap();

    }
}


