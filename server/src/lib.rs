use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref CONFIG: Config = {
        let config: Config = Config {
            workers: get_worker_env("WORKERS"),
            port: get_env_int("PORT"), 
            ws_port: get_env_int("WS_PORT"), 
            ws_url: get_env("WS_URL"), 
            twitch_chat_url: get_env("TWITCH_CHAT_URL"), 
            twitch_client_id: get_env("TWITCH_CLIENT_ID"), 
            twitch_client_secret: get_env("TWITCH_CLIENT_SECRET"), 
            twitch_access_token: get_env("TWITCH_ACCESS_TOKEN"), 
            twitch_oauth_token: get_env("TWITCH_OAUTH"), 
            twitch_refresh_token: get_env("TWITCH_REFRESH_TOKEN") 
        };
        config
    };
}

pub struct Config {
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

fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => return val,
        Err(_) => return "".to_string()
    }
}

fn get_env_int(key: &str) -> u16 {
    match env::var(key) {
        Ok(val) => match val.parse::<u16>() {
            Ok(n) => return n,
            Err(_) => return 0
        }
        Err(_) => return 0
    }
}

fn get_worker_env(key: &str) -> u8 {
    match env::var(key) {
        Ok(val) => match val.parse::<u8>() {
            Ok(n) => return n,
            Err(_) => return 1
        }
        Err(_) => return 1
    }
}