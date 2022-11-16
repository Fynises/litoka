use crate::twitch_websocket::twitch_ws::CONNECTION;
use lazy_static::lazy_static;
use regex::Regex;
use super::command_parser;

lazy_static! {
    pub static ref PRIVMSG_REGEX: Regex = Regex::new(r"user-type= [^\s]+.tmi\.twitch\.tv PRIVMSG #[^\s]+ :").unwrap();
    static ref PRIVMSG_CAPTURE_REGEX: Regex = Regex::new(r"user-type= [^\s]+.tmi\.twitch\.tv PRIVMSG #[^\s]+ :(.*)").unwrap();
}

pub fn parse_message(msg: &str) {
    if msg == "PING :tmi.twitch.tv\r\n" {
        CONNECTION.lock().unwrap().send_pong();
        return
    }

    //if message is a valid PRIVMSG then send to command parser
    if PRIVMSG_REGEX.is_match(&msg) {
        command_parser::capture_message(msg);
    }
}