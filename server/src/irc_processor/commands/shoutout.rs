use regex::Regex;
use lazy_static::lazy_static;
use serde::Serialize;
use log::{info, error};
use crate::client_websocket::shoutout_structs::ClientConnectOptions;
use crate::db::functions::clips::{get_clip};
use crate::irc_processor::command_parser::TwitchMessage;
use crate::client_websocket::session_map::SESSION;
use crate::db::functions::users;

lazy_static! {
    static ref TARGET_CHANNEL_CAPTURE: Regex = Regex::new(r"!so\s@?(\w+)").unwrap();
    static ref URL_CAPTURE: Regex = Regex::new(r"(.*)-preview-").unwrap();
}

#[derive(Serialize)]
pub struct SoClipData {
    pub clip_url: String,
    pub streamer: String,
    pub profile_pic: String,
    pub duration: f64,
}

pub async fn run_shoutout_command(msg: &TwitchMessage) {
    let session_map = SESSION.lock().unwrap();
    if session_map.has_channel(msg.channel.clone()) {
        let options = session_map.get_options_map(msg.channel.clone()).expect("unable to get map").to_owned();
        
        info!("client {} has {} connected sessions", msg.channel, options.keys().len());

        std::mem::drop(session_map); //TODO: try and find a way to unlock the mutex without manually dropping

        let target_channel_cap = TARGET_CHANNEL_CAPTURE.captures(&msg.message);
        let target_channel: String = match target_channel_cap {
            Some(cap) => match cap.len() {
                2 => cap[1].to_string(),
                _ => return
            },
            None => return
        };

        //validate the command
        for (key, val) in options.iter() {
            info!("allow mods? : {}", val.allow_mods); // debug
            if msg.is_broadcaster || (msg.is_mod && val.allow_mods) {
                execute_shoutout(target_channel.clone(), key, val).await;
            }
        }

    } else {
        error!("cannot find channel: {}", msg.channel);
        return
    }
}

//filtering will not be implemented for now
async fn execute_shoutout(target_channel: String ,client_uuid: &String, _options: &ClientConnectOptions) {

    let streamer = match users::get_user(target_channel.clone()).await {
        Some(res) => res,
        None => return
    };

    let clip =  match get_clip(streamer.id, _options.to_owned()).await {
        Some(res) => res,
        None => return
    };

    let clip_url = format_clip_url(clip.data.thumbnail_url.clone()).expect("error formatting clip url");
    let clip_duration = clip.data.duration;

    let clip_data: SoClipData = SoClipData { 
        clip_url, 
        streamer: streamer.login, 
        profile_pic: streamer.profile_image_url, 
        duration: clip_duration 
    };

    SESSION.try_lock().expect("error acquiring lock").send_clip(clip_data, client_uuid.clone());
    return;
}

fn format_clip_url(url: String) -> Option<String> {
    let url_cap = URL_CAPTURE.captures(&url).expect("error capturing url");
    let mut output_url = match url_cap.len() {
        2 => url_cap[1].to_string(),
        _ => return None
    };
    output_url.push_str(".mp4");
    return Some(output_url);
}