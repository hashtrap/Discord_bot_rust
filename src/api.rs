use std::sync::{LazyLock,atomic};
use reqwest::Client;
use std::env;

pub mod genius_api;
pub mod spotify_api;