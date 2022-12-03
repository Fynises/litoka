use awc::Client;
use serde::Deserialize;
use crate::{
    lib::CONFIG, 
    db::model::{
        clip::JsonClipData, 
        user::JsonUserData
    }
};
use log::error;

#[derive(serde::Serialize, Debug)]
struct StreamerIdQuery {
    login: String
}

#[derive(serde::Serialize, Debug)]
struct ClipsQuery {
    broadcaster_id: String,
    first: u8,
    after: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ApiGetClips {
    pub data: Vec<JsonClipData>,
    pub pagination: PaginationObj,
}

#[derive(Deserialize, Debug)]
pub struct PaginationObj {
    pub cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ApiGetUser {
    pub data: Vec<JsonUserData>,
}

pub async fn get_user_object(name: String) -> Option<ApiGetUser> {
    let client = Client::default();
    let req = client.get("https://api.twitch.tv/helix/users")
        .query(&StreamerIdQuery {login: name.into()})
        .unwrap()
        .insert_header(("Authorization", format!("Bearer {}", CONFIG.twitch_oauth_token)))
        .insert_header(("Client-Id", CONFIG.twitch_client_id.as_bytes()));
    match req.send().await {
        Ok(mut res) => match res.json::<ApiGetUser>().await {
            Ok(json) => return Some(json),
            Err(_) => return None,
        },
        Err(_) => return None,
    }
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
