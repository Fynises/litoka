use lazy_static::lazy_static;
use regex::Regex;

use super::commands::shoutout::run_shoutout_command;

lazy_static! {
    static ref PRIVMSG_CAPTURE_REGEX: Regex = Regex::new(r"user-type= [^\s]+.tmi\.twitch\.tv PRIVMSG #[^\s]+ :(.*)").unwrap();
    static ref CHATTER_CAPTURE: Regex = Regex::new(r";display-name=([^;]*);").unwrap();
    static ref CHATTERID_CAPTURE: Regex = Regex::new(r";user-id=([^;]*);").unwrap();
    static ref CHANNEL_CAPTURE: Regex = Regex::new(r"PRIVMSG #([^\s]+) :").unwrap();
    static ref ISBROADCASTER_CAPTURE: Regex = Regex::new(r"badges=broadcaster").unwrap();
    static ref ISMOD_CAPTURE: Regex = Regex::new(r";mod=1;").unwrap();
    static ref ISSUBSCRIBER_CAPTURE: Regex = Regex::new(r";subscriber=(.);").unwrap();
    static ref COMMAND_CAPTURE_REGEX: Regex = Regex::new(r"!(\w+)").unwrap();
}

#[derive(Clone)]
pub struct TwitchMessage {
    pub chatter_name: String,
    pub chatter_id: String,
    pub channel: String,
    pub is_broadcaster: bool,
    pub is_mod: bool,
    pub is_subscriber: bool,
    pub message: String
}

pub fn capture_message(privmsg: &str) {

    //chatter name
    let chatter_name_cap = CHATTER_CAPTURE.captures(&privmsg).expect("error capturing chatter name");
    let chatter_name: String = match chatter_name_cap.len() {
        1 => chatter_name_cap[1].to_string(),
        _ => return
    };

    //chatter id
    let chatter_id_cap = CHATTERID_CAPTURE.captures(&privmsg).expect("error capturing chatter id");
    let chatter_id: String = match chatter_id_cap.len() {
        1 => chatter_id_cap[1].to_string(),
        _ => return
    };

    let channel_cap = CHANNEL_CAPTURE.captures(&privmsg).expect("error capturing channel name");
    let channel: String = match channel_cap.len() {
        1 => channel_cap[1].to_string(),
        _ => return
    };

    let is_broadcaster = ISBROADCASTER_CAPTURE.is_match(&privmsg);

    let is_mod = ISMOD_CAPTURE.is_match(&privmsg);

    let is_subscriber = ISSUBSCRIBER_CAPTURE.is_match(&privmsg);

    let message_cap = PRIVMSG_CAPTURE_REGEX.captures(&privmsg).expect("error capturing message");
    let message: String = match message_cap.len() {
        1 => message_cap[1].to_string(),
        _ => return
    };

    let twitch_message: TwitchMessage = TwitchMessage { 
        chatter_name, 
        chatter_id, 
        channel, 
        is_broadcaster, 
        is_mod, 
        is_subscriber, 
        message 
    };

    parse_command(twitch_message);

}

//command parsing will be hardcoded for now, will plan on adding dynamic prefixes
fn parse_command(message: TwitchMessage) {
    let command = COMMAND_CAPTURE_REGEX.captures(&message.message).expect("regex capture error");
    match command.len() {
        1 => {
            match &command[1] {
                "so" => run_shoutout_command(&message),
                _ => ()
            }
        },
        _ => ()
    }
}

