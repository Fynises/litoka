use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use tokio::sync::mpsc::{self,UnboundedSender};
use log::info;

lazy_static! {
    pub static ref CONNECTION: Arc<Mutex<TwitchConnection>> = {
        let (temp_tx, _temp_rx) = mpsc::unbounded_channel();
        let twitch_connection: TwitchConnection = TwitchConnection { 
            sender: temp_tx, 
        };
        Arc::new(Mutex::new(twitch_connection))
    };
}

pub struct TwitchConnection {
    sender: UnboundedSender<String>,
}

impl TwitchConnection {
    
    pub async fn set_sender(&mut self, sender: UnboundedSender<String>) {
        self.sender = sender;
    }

    pub fn send_pong(&self) {
        self.sender.send("PONG :tmi.twitch.tv".to_string()).unwrap();
    }

    pub fn join(&self, channel_name: String) {
        info!("recieved join request: {}", channel_name);
        self.sender.send(format!("JOIN #{}", channel_name)).unwrap();
    }

}
