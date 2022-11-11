use lazy_static::lazy_static;
use std::env;
use std::fs;
use serde::Deserialize;

lazy_static! {
    pub static ref CONFIG: Config = load_config();
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub index_path: String,
    pub dist_path: String,
    pub workers: u8,
    pub port: u16,
    pub ws_port: u16,
    pub ws_url: String,
    pub twitch_chat_url: String,
    pub twitch_client_id: String,
    pub twitch_client_secret: String,
    pub twitch_access_token: String,
    pub twitch_oauth_token: String,
    pub twitch_refresh_token: String
}

impl Default for Config {
    fn default() -> Config {
        Config {
            index_path: "".to_string(),
            dist_path: "".to_string(),
            workers: 1,
            port: 0,
            ws_port: 0,
            ws_url: "".to_string(),
            twitch_chat_url: "".to_string(),
            twitch_client_id: "".to_string(),
            twitch_client_secret: "".to_string(),
            twitch_access_token: "".to_string(),
            twitch_oauth_token: "".to_string(),
            twitch_refresh_token: "".to_string()
        }
    }
}

fn load_config() -> Config {
    let data = fs::read_to_string(get_env("CONFIG_PATH")).expect("err");
    let config: Config = serde_json::from_str(&data).expect("Json read error");
    return config
}

fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => return val,
        Err(_) => return "".to_string()
    }
}