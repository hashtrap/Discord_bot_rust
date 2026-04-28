
use crate::api::*;
use crate::get_env_var;
use std::string::String;




static Spotify_Client: LazyLock<Client> = LazyLock::new(|| {

    Client::new()
});
pub  async fn retrieve_cred()->Result<String,reqwest::Error>
{

    let client_id= get_env_var("CLIENT_ID");
    let client_secret = get_env_var("CLIENT_SECRET");
    let credentials=Spotify_Client.post("https://accounts.spotify.com/api/token")
                    .header("Content-Type","application/x-www-form-urlencoded")
                    .body(format!("grant_type=client_credentials&client_id={client_id}&client_secret={client_secret}"))
                    .send().await?;
    let target=credentials.text().await?;


    Ok(target)
}