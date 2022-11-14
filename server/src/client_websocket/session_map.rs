use std::{
    sync::{Arc,Mutex}, 
    collections::HashMap
};
use lazy_static::lazy_static;
use tokio::sync::mpsc::UnboundedSender;

use super::shoutout_structs::ClientConnectOptions;

lazy_static! {
    pub static ref SESSION: Arc<Mutex<Sessions>> = {
        let sessions: Sessions = Sessions { 
            sessions: HashMap::new(), 
            channels: HashMap::new(),
            id_counter: 0,
        };
        Arc::new(Mutex::new(sessions))
    };
}

pub struct Sessions {
    pub sessions: HashMap<usize, UnboundedSender<String>>,
    pub channels: HashMap<String, HashMap<usize, ClientConnectOptions>>,
    id_counter: usize,
}

impl Sessions {
    pub fn add(&mut self, session: UnboundedSender<String>, options: ClientConnectOptions) -> usize {
        let channel_name: String = options.channel.clone();
        let id: usize = self.id_counter;
        self.increment_counter();

        if self.channels.contains_key(&channel_name) {            
            self.channels.get_mut(&channel_name).expect("err").insert(id, options);
        } else {
            self.channels.insert(channel_name, HashMap::from([(id, options)]));
        }
        self.sessions.insert(id, session);
        
        id
    }

    fn increment_counter(&mut self) {
        self.id_counter += 1;
    }

    pub fn close(&mut self, id: usize, channel: String) {
        self.sessions.remove(&id).unwrap();
        self.channels.get(&channel).unwrap().to_owned().remove(&id);
    }

}