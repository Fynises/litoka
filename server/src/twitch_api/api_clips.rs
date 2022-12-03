use awc::Client;
use log::error;
use serde::{Serialize, Deserialize};
use crate::{
    lib::CONFIG,
    db::model::clip::JsonClipData
};

#[derive(Serialize, Debug)]
struct ClipsQuery {
    broadcaster_id: String,
    first: u8,
    after: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PaginationObj {
    pub cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ApiGetClips {
    pub data: Vec<JsonClipData>,
    pub pagination: PaginationObj,
}

pub async fn api_fetch_clips(broadcaster_id: String, pagination: Option<String>) -> Option<ApiGetClips> {
    let client = Client::default();
    let req = client.get("https://api.twitch.tv/helix/clips")
        .query(&ClipsQuery {broadcaster_id, first: 100, after: pagination})
        .unwrap()
        .insert_header(("Authorization", format!("Bearer {}", CONFIG.twitch_oauth_token)))
        .insert_header(("Client-Id", CONFIG.twitch_client_id.as_bytes()));
    match req.send().await {
        Ok(mut res) => match res.json::<ApiGetClips>().await {
            Ok(json) => return Some(json),
            Err(err) => {
                error!("{}",err);
                return None
            },
        },
        Err(err) => {
            error!("{}", err);
            return None
        },
    }
}
