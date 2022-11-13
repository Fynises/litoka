use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ClientConnectOptions {
    pub channel: String,
    pub allow_mods: bool,
    pub filter_type: Option<String>,
    pub filter_params: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ClientConnectMessage {
    pub options: ClientConnectOptions,
    pub hash: String,
}

#[derive(Serialize)]
pub struct ClipData {
    pub clip_url: String,
    pub streamer: String,
    pub profile_pic: String,
    pub clip_duration: usize
}

impl Default for ClientConnectOptions {
    fn default() -> Self {
        ClientConnectOptions { 
            channel: "".to_string(), 
            allow_mods: true, 
            filter_type: Some("".to_string()), 
            filter_params: Some("".to_string()),
        }
    }
}

impl Default for ClientConnectMessage {
    fn default() -> Self {
        ClientConnectMessage { 
            options: ClientConnectOptions::default(), 
            hash: "".to_string(),
        }
    }
}