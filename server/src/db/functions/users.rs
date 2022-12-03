use crate::{
    db::{
        db_client::DB_CLIENT,
        model::user::{
            JsonUserData,
            User,
            UserData,
        }
    },
    twitch_api::api_users::api_fetch_user
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

pub async fn get_user(username: String) -> Option<UserData> {    
    match COLLECTION.find_one(doc! {"data.login" : &username}, None).await {
        Ok(Some(user)) => return Some(user.data),
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

pub async fn add_and_get_user(username: String) -> Option<UserData> {
    match api_fetch_user(username.clone()).await {
        Some(object) => {
            match object.data.len() {
                1 => {
                    let single_user = object.data.get(0).unwrap().to_owned();
                    insert_new_user(single_user.clone()).await;
                    return Some(UserData::from(single_user));
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

async fn insert_new_user(user_obj: JsonUserData) {
    match COLLECTION.insert_one(User::new_from_json(user_obj.clone()), None).await {
        Ok(_) => info!("added {} to db", user_obj.login),
        Err(_) => error!("failed to add {} to db", user_obj.login),
    }
}
