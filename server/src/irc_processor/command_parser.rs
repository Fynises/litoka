use lazy_static::lazy_static;
use regex::Regex;
use log::{error,info};

use super::commands::shoutout::run_shoutout_command;

lazy_static! {
    static ref PRIVMSG_OPTIONS_CAPTURE: Regex = Regex::new(r"(.*):[^\s]+.tmi\.twitch.tv PRIVMSG #([^\s]+) :(.*)").unwrap();
    static ref CHATTER_CAPTURE: Regex = Regex::new(r"display-name=([^;]*)").unwrap();
    static ref CHATTERID_CAPTURE: Regex = Regex::new(r"user-id=([^;]*)").unwrap();
    static ref ISBROADCASTER_CAPTURE: Regex = Regex::new(r"badges=broadcaster").unwrap();
    static ref ISMOD_CAPTURE: Regex = Regex::new(r"mod=1").unwrap();
    static ref ISSUBSCRIBER_CAPTURE: Regex = Regex::new(r"subscriber=1").unwrap();
    static ref ISVIP_CAPTURE: Regex = Regex::new(r"vip=1").unwrap();
    static ref COMMAND_CAPTURE_REGEX: Regex = Regex::new(r"!(\w+)").unwrap();
}

#[derive(Clone, Debug)]
pub struct TwitchMessage {
    pub chatter_name: String,
    pub chatter_id: String,
    pub channel: String,
    pub is_broadcaster: bool,
    pub is_mod: bool,
    pub is_subscriber: bool,
    pub is_vip: bool,
    pub message: String
}

pub async fn capture_message(privmsg: &str) {

    let privmsg_options: String;
    let channel: String;
    let message: String;

    match PRIVMSG_OPTIONS_CAPTURE.captures(&privmsg) {
        Some(captures) => {
            match captures.len() {
                4 => {
                    privmsg_options = captures[1].to_string();
                    channel = captures[2].to_string();
                    let mut msg = captures[3].to_string();
                    msg.pop();
                    message = msg;
                }
                _ => {
                    error!("too many or too few captures for PRIVMSG");
                    return
                }
            }
        },
        None => {
            error!("error in parsing PRIVMSG");
            return
        }
    }

    let chatter_name = match CHATTER_CAPTURE.captures(&privmsg_options) {
        Some(captures) => {
            match captures.len() {
                2 => captures[1].to_string(),
                _ => return
            }
        },
        None => {
            error!("error in capturing chatter name");
            return
        }
    };

    let chatter_id = match CHATTERID_CAPTURE.captures(&privmsg_options) {
        Some(captures) => {
            match captures.len() {
                2 => captures[1].to_string(),
                _ => return
            }
        },
        None => {
            error!("error in capturing chatter id");
            return
        }
    };

    let is_broadcaster = ISBROADCASTER_CAPTURE.is_match(&privmsg_options);

    let is_mod = ISMOD_CAPTURE.is_match(&privmsg_options);

    let is_subscriber = ISSUBSCRIBER_CAPTURE.is_match(&privmsg_options);

    let is_vip = ISVIP_CAPTURE.is_match(&privmsg_options);

    let twitch_message: TwitchMessage = TwitchMessage { 
        chatter_name, 
        chatter_id, 
        channel, 
        is_broadcaster, 
        is_mod, 
        is_subscriber, 
        is_vip,
        message 
    };

    // debug show twitch message
    info!("{twitch_message:?}");

    parse_command(twitch_message).await;

}

//command parsing will be hardcoded for now, will plan on adding dynamic prefixes
async fn parse_command(message: TwitchMessage) {
    let command = match COMMAND_CAPTURE_REGEX.captures(&message.message) {
        Some(capture) => capture,
        None => {
            return
        }
    };
    match command.len() {
        2 => {
            match &command[1] {
                "so" => run_shoutout_command(&message).await,
                _ => return
            }
        },
        _ => return
    }
}

