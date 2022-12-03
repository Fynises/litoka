use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct ClientConnectOptions {
    pub channel: String,
    pub allow_mods: bool,
    pub filter_type: Option<String>,
    pub filter_params: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct ClientConnectMessage {
    pub options: ClientConnectOptions,
    pub hash: String,
}

#[derive(Serialize, Debug)]
pub struct ClipData {
    pub clip_url: String,
    pub streamer: String,
    pub profile_pic: String,
    pub clip_duration: usize,
}

impl Default for ClientConnectOptions {
    fn default() -> Self {
        ClientConnectOptions {
            channel: "".to_string(),
            allow_mods: true,
            filter_type: None,
            filter_params: None,
        }
    }
}

impl ClientConnectOptions {
    pub fn parse_filter(self) -> FilterType {
        match self.filter_type {
            Some(ftype) => match ftype.as_ref() {
                "topViewed" => FilterType::TopViewed { param: self.filter_params.unwrap() },
                "number" => FilterType::NumberRecent { param: self.filter_params.unwrap() },
                "time" => FilterType::Time { param: self.filter_params.unwrap() },
                _ => {
                    error!("error parsing filter type, defaulting to fullrandom");
                    return FilterType::FullRandom
                }
            },
            None => FilterType::FullRandom,
        }
    }
}


pub enum FilterType {
    FullRandom, //default
    TopViewed { param: i64 },
    NumberRecent { param: i64 },
    Time { param: i64 },
}

impl Default for ClientConnectMessage {
    fn default() -> Self {
        ClientConnectMessage {
            options: ClientConnectOptions::default(),
            hash: "".to_string(),
        }
    }
}
