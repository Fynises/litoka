use awc::Client;
use serde::Deserialize;
use crate::{
    lib::CONFIG,
    db::model::user::JsonUserData
};

#[derive(serde::Serialize, Debug)]
struct StreamerIdQuery {
    login: String
}

#[derive(Deserialize, Debug)]
pub struct ApiGetUser {
    pub data: Vec<JsonUserData>,
}

pub async fn api_fetch_user(name: String) -> Option<ApiGetUser> {
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