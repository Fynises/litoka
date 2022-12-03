use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Clip {
    pub data: ClipData,
    pub date_last_checked: DateTime,
}

impl Clip {
    pub fn new_from_json(json_clip_data: JsonClipData) -> Self {
        Self {
            data: ClipData::from(json_clip_data),
            date_last_checked: DateTime::now(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonClipData {
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
    #[serde(with = "bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
    pub thumbnail_url: String,
    pub duration: f64,
    pub vod_offset: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ClipData {
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
    pub created_at: DateTime,
    pub thumbnail_url: String,
    pub duration: f64,
    pub vod_offset: Option<usize>,
}

impl From<JsonClipData> for ClipData {
    fn from(other: JsonClipData) -> ClipData {
        ClipData { 
            id: other.id, 
            url: other.url, 
            embed_url: other.embed_url, 
            broadcaster_id: other.broadcaster_id, 
            broadcaster_name: other.broadcaster_name, 
            creator_id: other.creator_id, 
            creator_name: other.creator_name, 
            video_id: other.video_id, 
            game_id: other.game_id, 
            language: other.language, 
            title: other.title, 
            view_count: other.view_count, 
            created_at: other.created_at, 
            thumbnail_url: other.thumbnail_url, 
            duration: other.duration,
            vod_offset: other.vod_offset 
        }
    }
}

impl From<ClipData> for JsonClipData {
    fn from (other: ClipData) -> JsonClipData {
        JsonClipData { 
            id: other.id, 
            url: other.url, 
            embed_url: other.embed_url, 
            broadcaster_id: other.broadcaster_id, 
            broadcaster_name: other.broadcaster_name, 
            creator_id: other.creator_id, 
            creator_name: other.creator_name, 
            video_id: other.video_id, 
            game_id: other.game_id, 
            language: other.language, 
            title: other.title, 
            view_count: other.view_count, 
            created_at: other.created_at, 
            thumbnail_url: other.thumbnail_url, 
            duration: other.duration, 
            vod_offset: other.vod_offset 
        }
    }
}
