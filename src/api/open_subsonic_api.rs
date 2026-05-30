#![allow(warnings)]

use std::io::BufRead;
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
    artist: String,
    album: String,
    duration:u32
}
#[derive(Debug)]
struct Song
{
    title:String,
    artist:String,
    album:String,
    duration:u32,
}

/*
Simple functions below that help with the easiness of the async functions like one that does hashing for the password when i want to test connectivity
or another that retrieves the credentials so i can use that way for multiple functions
 */
fn return_creds()->(String,String,String,String)
{
    let url=get_env_var("URL");
    let username=get_env_var("USERNAME");
    let name=String::from("Hazbin_Motel");
    let password=get_env_var("PASSWORD");
    (url,username,name,password)

}

fn hashing()->(String,String)
{
    let password=get_env_var("PASSWORD");
    let salt=get_env_var("SALTI");
    let hash = md5::compute(format!("{}{}",password,salt));
    (hash.iter().map(|&b| format!("{:02x}", b)).collect(),salt)

}

fn random_song(mut playlist:  Vec<Song>)->Song
{
    use rand::prelude::*;

    let mut rng=rand::rng();
    let playlist_length=playlist.len();

    let rand=rng.random_range(0..playlist_length);

    let song=playlist.remove(rand);
    println!("The song chosen is: {:#?}",&song);
    song

}


/*
Below are the main async functions that will do the business logic of the song functionalities
 */

async fn get_lyrics(client:&reqwest::Client,song:Song)->Result<Vec<String>,reqwest::Error>
{
    let title=song.title;
    let artist=song.artist;
    let album=song.album;
    let duration=song.duration;
    let url=get_env_var("URL_LYRICS");
    let response=client.get(format!("{url}/get?artist_name={artist}\
                                  &track_name={title}&album_name={album}&duration={duration}"))
                                 .send().await?;


    println!("Response get_lyrics status: {:?}",&response.status());
    let data: Value = response.json().await?;
    let lyric_data=data["plainLyrics"].as_str().unwrap();
    //println!("{:?}",&lyric_data);
    let lyrics:Vec<String> = lyric_data .split('\n')
        .map(String::from)
        .collect();
    //println!("{:#?}",lyrics[1]);
    Ok(lyrics)
}
async fn get_playlist(client:&reqwest::Client)->Result<(Vec<Song>),reqwest::Error>
{
    let playlist_id = get_env_var("PLAYLIST_ID");
    let (url, username, name,password) = return_creds();
    let response = client.get(format!("{url}/rest/getPlaylist.view?id={playlist_id}\
                                  &u={username}&p={password}\
                                  &f=json&v=1.13.0&c={name}"))
        .send().await?;

    println!("Response get_playlist status: {:?}", response.status());
    let text = response.text().await?;
    let data: Response = serde_json::from_str(&text).unwrap();

    let song: Vec<Song> = data
        .subsonic_response
        .playlist
        .entry
        .into_iter()
        .map(|e| Song{ title:e.title,artist:e.artist,album:e.album,duration:e.duration } )
        .collect();



    Ok(song)
}

pub async fn daily_duet()->Result<Vec<String>,reqwest::Error>
{
    let client = reqwest::Client::new();
    let client2 = client.clone();

    let playlist=match get_playlist(&client)

    .await
    {
        Ok(songs) =>

            {
                if songs.len() == 0
                {
                    panic!("Error while retrieving playlist, the playlist was found empty");
                }

                else
                {
                    songs
                }
            },

        Err(err)=>panic!("Error while connecting to the playlist of the server, {}",err)
    };

    let song_of_day=random_song(playlist);

    let lyrics=match get_lyrics(&client2,song_of_day).await
    {
        Ok(lyrics) =>
            {
                if lyrics.len() == 0
                {
                    panic!("Error, could not find lyrics");
                }

                else
                {
                    lyrics
                }
            },
        Err(err)=>panic!("Error while connecting to the lyrics API, {}",err)
    };


    Ok(lyrics)

}



/*
Everything below this comment are functions that are used for unit testing the API they are not to be messed with and dont get compiled when using
cargo  build or cargo run
 */

#[cfg(test)]
mod tests
{
    #![allow(warnings)]

    use reqwest::Error;
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



        let response=client.post("https://music.dstefani.site/rest/ping.view")
            .header("Content-Type"," application/x-www-form-urlencoded")
            .body(format!("c=Hazbin_Motel&v=1.13.0&u={:?}&t={:?}&s={:?}&f=json",username,token,salt))
            .send().await?;

        println!("Response status: {:?}",&response.status());
        println!("Response full:{:?}",response.text().await.expect("Response recieved but content is empty"));

        Ok(())

    }


    #[tokio::test]
    async fn playlist_retrieval()
    {

        prepare_env();

        let client=reqwest::Client::new();

        let result=get_playlist(&client).await.unwrap();

        println!("{:#?}",result);


    }
    #[tokio::test]
    async fn choose_song()
    {

        prepare_env();

        let client = reqwest::Client::new();
        let songs=match get_playlist(&client).await
        {
            Ok(list) => list,
            Err(e)=>panic!("{:?}", e),
        };

        let song=random_song(songs);
        println!("{:#?}", song);

    }

    #[tokio::test]
    async fn lyrics()
    {

        prepare_env();

        let client = reqwest::Client::new();

        let song = Song {
            artist: String::from("Borislav Slavov"),
            title: String::from("I Want to Live"),
            album: String::from("Baldur's Gate 3 (Original Game Soundtrack)"),
            duration: 233,
        };



        println!("{:#?}",&song);

        get_lyrics(&client,song).await.unwrap();
    }

    #[tokio::test]
    async fn duet_test()
    {

        prepare_env();
        let test_result=daily_duet().await;

        println!("{:#?}",test_result);
    }


}


