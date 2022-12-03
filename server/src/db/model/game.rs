use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub box_art_url: String,
    pub igdb_id: String
}