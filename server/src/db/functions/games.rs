use crate::{
    db::{db_client::DB_CLIENT, model::game::Game}, twitch_api::api_games::api_fetch_game,
};
use lazy_static::lazy_static;
use log::{error, warn, info};
use mongodb::{bson::doc, Collection};

lazy_static! {
    static ref COLLECTION: Collection<Game> = DB_CLIENT
        .get()
        .unwrap()
        .database("main")
        .collection("games");
}

pub async fn get_game_from_db(game_id: String) -> Option<Game> {
    match COLLECTION.find_one(doc! {"id": game_id.clone()}, None).await{
        Ok(game_option) => match game_option {
            Some(game) => Some(game),
            None => {
                warn!("game id {} not found in db", game_id);
                return get_and_add_game(game_id).await;
            }
        },
        Err(err) => {
            error!("{}", err);
            return None;
        }
    }
}

async fn get_and_add_game(game_id: String) -> Option<Game> {
    match api_fetch_game(game_id.clone()).await {
        Some(game) => match game.data.len() {
            1 => {
                let single_game = game.data.get(0).unwrap().to_owned();
                insert_new_game(&single_game).await;
                return Some(single_game);
            },
            _ => {
                warn!("zero or too many games found for id {}", game_id);
                return None;
            }
        },
        None => {
            warn!("game id {} not returned from twitch api", game_id);
            return None;
        }
    }
}

async fn insert_new_game(game: &Game) {
    match COLLECTION.insert_one(game.clone(), None).await{
        Ok(_) => info!("added {} [{}] to db", game.name, game.id),
        Err(_) => error!("failed to add {} [{}] to db", game.name, game.id),
    }
}
