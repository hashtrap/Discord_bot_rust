use serenity::all::Color;

pub fn menu_creator_daily(text_option:Option<String>)
{

    let menu=serenity::builder::CreateEmbed::new();
    let menu=menu.color(Color::from_rgb(255, 49, 49));

    menu.description(text_option.unwrap_or(String::from("Start a duet today with a random or a song of your choice 🎵")));



}

pub fn menu_creator_duet(songs:Option<String>)
{

    if let Some(songs) = songs
    {

    }

    else
    {
        println!("No songs present.");
    }

    let menu=serenity::builder::CreateEmbed::new();
    let menu=menu.color(Color::from_rgb(255, 49, 49));

    //menu.description(text_option.unwrap_or(String::from("Start a duet today with a random or a song of your choice 🎵")));



}