use std::{
    sync::{Arc,Mutex}, 
    collections::HashMap
};
use lazy_static::lazy_static;
use tokio::sync::mpsc::UnboundedSender;
use log::{info, error};
use crate::irc_processor::commands::shoutout::SoClipData;
use crate::twitch_websocket::twitch_ws::CONNECTION;
use super::shoutout_structs::ClientConnectOptions;

lazy_static! {
    pub static ref SESSION: Arc<Mutex<Sessions>> = {
        let sessions: Sessions = Sessions { 
            sessions: HashMap::new(), 
            channels: HashMap::new(),
        };
        Arc::new(Mutex::new(sessions))
    };
}

pub struct Sessions {
    pub sessions: HashMap<String, UnboundedSender<String>>,
    pub channels: HashMap<String, HashMap<String, ClientConnectOptions>>,
}

impl Sessions {
    pub fn add(&mut self, session: UnboundedSender<String>, options: ClientConnectOptions, uuid: String) {
        let channel_name: String = options.channel.clone();

        if self.channels.contains_key(&channel_name) {            
            self.channels.get_mut(&channel_name).expect("err").insert(uuid.clone(), options);
        } else {
            self.channels.insert(channel_name.clone(), HashMap::from([(uuid.clone(), options)]));
        }
        self.sessions.insert(uuid.clone(), session);

        CONNECTION.lock().unwrap().join(channel_name);
    }

    pub fn get_options_map(&self, channel: String) -> Option<&HashMap<String, ClientConnectOptions>> {
        return self.channels.get(&channel);
    }

    pub fn has_channel(&self, channel: String) -> bool {
        return self.channels.contains_key(&channel);
    }

    pub fn send_clip(&self, clip_data: SoClipData, client_uuid: String) {
        let tx = match self.sessions.get(&client_uuid) {
            Some(sender) => sender,
            None => {
                error!("error retrieving sender from [{}]", client_uuid);
                return
            }
        };
        tx.send(serde_json::to_string(&clip_data).expect("error serialising clip data")).expect("error sending to client");
    }

    pub async fn close(&mut self, uuid: String, channel: String) {
        info!("removing session {} from channel {}" , uuid, channel);
        match self.sessions.remove_entry(&uuid) {
            Some(item) => info!("successfully removed [{}] from session map", item.0),
            None => error!("entry does not exist from session map"),
        };

        let channels_map = self.channels.get_mut(&channel).unwrap();

        match channels_map.remove_entry(&uuid) {
            Some(item) => info!("successfully removed [{}] from channel map", item.0),
            None => error!("entry does not exist from channel map"),
        };
        
        if channels_map.is_empty() {
            info!("session map for channel {} is empty, deleing", channel);
            self.channels.remove_entry(&channel);
        }
    }

}