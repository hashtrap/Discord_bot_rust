#![allow(warnings)]
use md5;
use crate::api::*;
use crate::get_env_var;

async fn connect_server(client:&reqwest::Client)->Result<(),reqwest::Error>
{
    let username=get_env_var("USERNAME");
    //println!("Username: {}",&username);
    let (token,salt)=hashing();

    println!("Salt: {}",&salt);

    let response=client.post("https://music.dstefani.site/rest/ping.view")
          .header("Content-Type"," application/x-www-form-urlencoded")
          .body(format!("c=Hazbin_Motel&v=1.13.0&u={:?}&t={:?}&s={:?}&f=json",username,token,salt))
          .send().await?;

    println!("Response status: {:?}",&response.status());
    todo!()
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
    async fn test_connection()
    {

        prepare_env();

        let client=reqwest::Client::new();
        connect_server(&client).await.unwrap();

    }


    #[tokio::test]
    async fn test_playlist_retrieval()
    {

        prepare_env();

        let client=reqwest::Client::new();


    }
}


