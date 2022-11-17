use awc::Client;
use serde::Deserialize;

use crate::lib::CONFIG;

#[derive(serde::Serialize, Debug)]
struct StreamerIdQuery {
    login: String
}

#[derive(serde::Serialize, Debug)]
struct ClipsQuery {
    broadcaster_id: String,
    first: u8,
}

#[derive(Deserialize, Debug)]
pub struct ApiGetClips {
    pub data: Vec<ApiSingleClip>,
    pub pagination: PaginationObj,
}

#[derive(Deserialize, Debug)]
pub struct PaginationObj {
    pub cursor: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ApiSingleClip {
    pub id: String,
    pub url: String,
    pub embed_url: String,
    pub broadcaster_id: String,
    pub broadcaster_name: String,
    pub creator_id: String,
    pub creator_name: String,
    pub video_id: String,
    pub game_id: String,
    pub language: String,
    pub title: String,
    pub view_count: usize,
    pub created_at: String,
    pub thumbnail_url: String,
    pub duration: f64,
}

#[derive(Deserialize, Debug)]
pub struct ApiGetUser {
    pub data: Vec<ApiSingleUser>,
}

#[derive(Deserialize, Debug)]
pub struct ApiSingleUser {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub r#type: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: usize,
    pub created_at: String,
}

pub async fn get_target_streamer_id(name: String) -> Option<ApiGetUser> {
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

pub async fn fetch_clips(broadcaster_id: String) -> Option<ApiGetClips> {
    let client = Client::default();
    let req = client.get("https://api.twitch.tv/helix/clips")
        .query(&ClipsQuery {broadcaster_id, first: 100})
        .unwrap()
        .insert_header(("Authorization", format!("Bearer {}", CONFIG.twitch_oauth_token)))
        .insert_header(("Client-Id", CONFIG.twitch_client_id.as_bytes()));
    match req.send().await {
        Ok(mut res) => match res.json::<ApiGetClips>().await {
            Ok(json) => return Some(json),
            Err(_) => return None,
        },
        Err(_) => return None,
    }
}