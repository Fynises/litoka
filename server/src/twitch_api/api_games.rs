use awc::Client;
use log::error;
use serde::{Deserialize, Serialize};
use crate::{
    lib::CONFIG,
    db::model::game::Game
};

#[derive(Serialize, Debug)]
struct GameQuery {
    id: String
}

#[derive(Deserialize, Debug)]
pub struct ApiGetGame {
    pub data: Vec<Game>
}

pub async fn api_fetch_game(game_id: String) -> Option<ApiGetGame> {
    let client = Client::default();
    let req = client.get("https://api.twitch.tv/helix/games")
        .query(&GameQuery { id: game_id })
        .unwrap()
        .insert_header(("Authorization", format!("Bearer {}", CONFIG.twitch_oauth_token)))
        .insert_header(("Client-Id", CONFIG.twitch_client_id.as_bytes()));
    match req.send().await{
        Ok(mut res) => match res.json::<ApiGetGame>().await {
            Ok(json) => return Some(json),
            Err(err) => {
                error!("{}", err);
                return None
            },
        },
        Err(err) => {
            error!("{}", err);
            return None
        },
    }
}