use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub data: UserData,
    pub date_last_checked: DateTime
}

impl User {
    pub fn new_from_json(json_user_data: JsonUserData) -> Self {
        Self { 
            data: UserData::from(json_user_data), 
            date_last_checked: DateTime::now(), 
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonUserData {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub r#type: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: Option<usize>,
    #[serde(with = "bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserData {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub r#type: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: Option<usize>,
    pub created_at: DateTime
}

impl From<JsonUserData> for UserData {
    fn from(other: JsonUserData) -> UserData {
        UserData { 
            id: other.id, 
            login: other.login, 
            display_name: other.display_name, 
            r#type: other.r#type, 
            broadcaster_type: other.broadcaster_type, 
            description: other.description, 
            profile_image_url: other.profile_image_url, 
            offline_image_url: other.offline_image_url, 
            view_count: other.view_count, 
            created_at: other.created_at 
        }
    }
}

impl From<UserData> for JsonUserData {
    fn from(other: UserData) -> JsonUserData {
        JsonUserData { 
            id: other.id, 
            login: other.login, 
            display_name: other.display_name, 
            r#type: other.r#type, 
            broadcaster_type: other.broadcaster_type, 
            description: other.description, 
            profile_image_url: other.profile_image_url, 
            offline_image_url: other.offline_image_url, 
            view_count: other.view_count, 
            created_at: other.created_at 
        }
    }
}