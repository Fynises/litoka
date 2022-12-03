use crate::{
    client_websocket::shoutout_structs::{ClientConnectOptions, FilterType},
    db::{
        db_client::DB_CLIENT, 
        model::clip::{Clip, JsonClipData}
    }, 
    twitch_api::twitch_api_controller::{api_fetch_clips},
};
use chrono::{Duration, Utc};
use futures::stream::TryStreamExt;
use lazy_static::lazy_static;
use log::{error, warn, info};
use mongodb::{
    bson::{doc, DateTime, Document},
    options::FindOptions,
    Collection,
};
use rand::Rng;

lazy_static! {
    static ref COLLECTION: Collection<Clip> = DB_CLIENT
        .get()
        .unwrap()
        .database("main")
        .collection("clips");
}

pub async fn get_clip(broadcaster_id: String, options: ClientConnectOptions) -> Option<Clip> {
    let filter_type = options.parse_filter();
    let query = get_filter(&filter_type, &broadcaster_id);
    let query_options = get_find_options(&filter_type);

    warn!("{query:?}");
    warn!("{query_options:?}");

    match fetch_clips(query, query_options).await {
        Some(clips) => {
            let rng = rand::thread_rng().gen_range(0..clips.len());
            return Some(clips.get(rng).expect("error extracting clip from map").clone())
        },
        None => {
            warn!("no clips found for {} in db", broadcaster_id);
            insert_all_clips(broadcaster_id).await;
            return None
        },
    };
}

pub async fn fetch_clips(filter: Option<Document>, options: Option<FindOptions>) -> Option<Vec<Clip>> {
    match COLLECTION.find(filter, options).await {
        Ok(cursor) => match cursor.try_collect::<Vec<Clip>>().await {
            Ok(clips) => match clips.len() {
                0 => {
                    warn!("no clips were found");
                    return None
                },
                _ => {
                    info!("fetched {} clips from db", clips.len());
                    return Some(clips)
                }
            },
            Err(_) => None,
        },
        Err(_) => {
            error!("error getting clip from DB");
            return None;
        }
    }
}

fn get_filter(filter_type: &FilterType, broadcaster_id: &String) -> Option<Document> {
    match filter_type {
        FilterType::Time { param } => {
            let date_from = Utc::now() - Duration::days(*param);
            let date_string = date_from.to_rfc3339();
            Some(doc! {"data.broadcaster_id": broadcaster_id, "data.created_at": doc!{"$gt": DateTime::parse_rfc3339_str(date_string).unwrap()} })
        }
        _ => Some(doc! {"data.broadcaster_id": broadcaster_id}),
    }
}

fn get_find_options(filter_type: &FilterType) -> Option<FindOptions> {
    match filter_type {
        FilterType::FullRandom => None,
        FilterType::TopViewed { param } => Some(
            FindOptions::builder()
                .sort(doc! {"view_count": -1})
                .limit(*param)
                .build(),
        ),
        FilterType::NumberRecent { param } => Some(
            FindOptions::builder()
                .sort(doc! {"created_at": -1})
                .limit(*param)
                .build(),
        ),
        FilterType::Time { param } => None,
    }
}

async fn insert_all_clips(broadcaster_id: String) {
    let mut clips: Vec<Clip> = Vec::new();

    let mut current_pagination: Option<String> = None;

    loop {
        match api_fetch_clips(broadcaster_id.clone(), current_pagination.clone()).await {
            Some(api_clips) => {
                clips.append(&mut parse_into_model(api_clips.data));
                match api_clips.pagination.cursor {
                    Some(pagination) => current_pagination = Some(pagination),
                    None => break,
                };
            },
            None => {
                warn!("no clips were found for {}", broadcaster_id);
                return
            },
        };
    }

    match COLLECTION.insert_many(clips.clone(), None).await {
        Ok(_) => {
            info!("successfully inserted {} into db", clips.len());
        },
        Err(_) => error!("error inserting all clips to db"),
    };

}

fn parse_into_model(api_clip: Vec<JsonClipData>) -> Vec<Clip> {
    let mut clips: Vec<Clip> = Vec::new();
    for clip in api_clip.iter() {
        clips.push(Clip::new_from_json(clip.clone()))
    };
    return clips;
}
