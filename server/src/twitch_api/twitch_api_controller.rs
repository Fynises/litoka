use awc::Client;
use serde::Deserialize;

use crate::lib::CONFIG;

#[derive(serde::Serialize)]
struct StreamerIdQuery {
    login: String
}

#[derive(Deserialize)]
struct ApiGetUser {
    id: String,
    login: String,
    r#type: String,
    broadcaster_type: String,
    description: String,
    view_count: usize,
    created_at: String
}

pub async fn get_target_streamer_id(name: String) {
    let mut client = Client::default();
    let req = client.get("https://api.twitch.tv/helix/users")
        .query(&StreamerIdQuery {login: name.into()})
        .unwrap()
        .insert_header(("Authorization", format!("Bearer {}", CONFIG.twitch_oauth_token)))
        .insert_header(("Client-Id", CONFIG.twitch_client_id.as_bytes()));

    let res = req.send().await.unwrap().json::<ApiGetUser>().await;
}