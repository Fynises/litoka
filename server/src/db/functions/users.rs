use crate::{
    db::{
        db_client::DB_CLIENT,
        model::user::{
            JsonUserData,
            User,
        }
    },
    twitch_api::{api_users::api_fetch_user, api_clips::api_fetch_clips}
};
use lazy_static::lazy_static;
use log::{error, info, warn};
use mongodb::{
    bson::doc,
    Collection,
};


lazy_static! {
    static ref COLLECTION: Collection<User> = DB_CLIENT.get().unwrap().database("main").collection("users");
}

pub async fn get_user(username: String) -> Option<User> {    
    match COLLECTION.find_one(doc! {"data.login" : &username}, None).await {
        Ok(Some(user)) => return Some(user),
        Ok(None) => {
            warn!("{} not found in DB, adding", &username);
            return add_and_get_user(username).await;
        }
        Err(_) => {
            error!("");
            return None;
        }
    }
}

pub async fn add_and_get_user(username: String) -> Option<User> {
    match api_fetch_user(username.clone()).await {
        Some(object) => {
            match object.data.len() {
                1 => {
                    let single_user = object.data.get(0).unwrap().to_owned();
                    let has_clips = match api_fetch_clips(single_user.id.clone(), None).await {
                        Some(clips) => match clips.data.len() {
                            0 => Some(false),
                            _ => Some(true),
                        },
                        None => Some(false),
                    };
                    
                    insert_new_user(single_user.clone(), has_clips).await;
                    return Some(User::new_from_json(single_user, has_clips));
                }
                _ => {
                    error!("zero or too many users were found for {}", username.clone());
                    return None;
                }
            };
        }
        None => {
            error!("username not found");
            return None;
        }
    };
}

async fn insert_new_user(user_obj: JsonUserData, has_clips: Option<bool>) {
    match COLLECTION.insert_one(User::new_from_json(user_obj.clone(), has_clips), None).await {
        Ok(_) => info!("added {} to db", user_obj.login),
        Err(_) => error!("failed to add {} to db", user_obj.login),
    }
}
