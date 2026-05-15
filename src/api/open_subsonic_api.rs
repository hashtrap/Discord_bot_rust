#![allow(warnings)]
use md5;
use crate::api::*;
use crate::get_env_var;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(rename = "subsonic-response")]
    subsonic_response: SubsonicResponse,
}

#[derive(Debug, Deserialize)]
struct SubsonicResponse {
    playlist: Playlist,
}

#[derive(Debug, Deserialize)]
struct Playlist {
    entry: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    title: String,
}


async fn get_playlist(client:reqwest::Client)->Result<(Vec<String>),reqwest::Error>
{
    let playlist_id=get_env_var("PLAYLIST_ID");
    let (url,username,name)=return_creds();
    let password=get_env_var("PASSWORD");
    let response=client.get(format!("{url}/rest/getPlaylist.view?id={playlist_id}\
                                  &u={username}&p={password}\
                                  &f=json&v=1.13.0&c={name}"))
                                 .send().await?;

    println!("Response get_playlist status: {:?}",response.status());
    let text = response.text().await?;
    let data: Response = serde_json::from_str(&text).unwrap();

    let song_names: Vec<String> = data
        .subsonic_response
        .playlist
        .entry
        .into_iter()
        .map(|e| e.title)
        .collect();

    println!("{:#?}", song_names);

    todo!()
}

fn return_creds()->(String,String,String)
{
    let url=get_env_var("URL");
    let username=get_env_var("USERNAME");
    let (password,salt)=hashing();
    let name=String::from("Hazbin_Motel");
    (url,username,name)

}

fn hashing()->(String,String)
{
    let password=get_env_var("PASSWORD");
    //println!("Password: {}",&password);
    let salt=get_env_var("SALTI");
    //println!("Salt: {}",&salt);
    let hash = md5::compute(format!("{}{}",password,salt));
    (hash.iter().map(|&b| format!("{:02x}", b)).collect(),salt)

}



#[cfg(test)]
mod tests
{
    #![allow(warnings)]
    use crate::prepare_env;
    use super::*;

    #[tokio::test]
    async fn connect_server()->Result<(),reqwest::Error>
    {
        prepare_env();

        let client = reqwest::Client::new();
        let username=get_env_var("USERNAME");
        //println!("Username: {}",&username);
        let (token,salt)=hashing();

        //println!("Salt: {}",&salt);

        let response=client.post("https://music.dstefani.site/rest/ping.view")
            .header("Content-Type"," application/x-www-form-urlencoded")
            .body(format!("c=Hazbin_Motel&v=1.13.0&u={:?}&t={:?}&s={:?}&f=json",username,token,salt))
            .send().await?;

        println!("Response status: {:?}",&response.status());
        println!("Response full:{:?}",response.text().await.expect("Response recieved but content is empty"));

        Ok(())

    }


    #[tokio::test]
    async fn test_playlist_retrieval()
    {

        prepare_env();

        let client=reqwest::Client::new();

        get_playlist(client).await.unwrap();


    }
}


