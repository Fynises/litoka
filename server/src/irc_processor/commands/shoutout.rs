use regex::Regex;
use lazy_static::lazy_static;
use crate::client_websocket::shoutout_structs::ClientConnectOptions;
use crate::irc_processor::command_parser::TwitchMessage;
use crate::client_websocket::session_map::SESSION;

lazy_static! {
    static ref TARGET_CHANNEL_CAPTURE: Regex = Regex::new(r"!so\s@?(\w+)").unwrap();
}

pub fn run_shoutout_command(msg: &TwitchMessage) {
    let session_map = SESSION.lock().unwrap();
    if session_map.has_channel(msg.channel.clone()){
        let options = session_map.get_options_map(msg.channel.clone()).unwrap();

            let target_channel_cap = TARGET_CHANNEL_CAPTURE.captures(&msg.message);
            let target_channel: String = match target_channel_cap {
                Some(cap) => match cap.len() {
                    1 => cap[1].to_string(),
                    _ => return
                },
                None => return
            };

        //validate the command
        for (key, val) in options.iter() {
            if msg.is_broadcaster || (msg.is_mod && val.allow_mods) {
                execute_shoutout(target_channel.clone(), key, val);
            }
        }

    } else {
        return
    }
}

//filtering will not be implemented for now
fn execute_shoutout(target_channel: String ,client_id: &usize, _options: &ClientConnectOptions) {

}

fn run_unfiltered() {

}