use rand::Rng;
use regex::Regex;
use lazy_static::lazy_static;
use serde::Serialize;
use log::{info, error};
use crate::client_websocket::shoutout_structs::ClientConnectOptions;
use crate::irc_processor::command_parser::TwitchMessage;
use crate::client_websocket::session_map::SESSION;
use crate::twitch_api::twitch_api_controller::{get_target_streamer_id, fetch_clips};

lazy_static! {
    static ref TARGET_CHANNEL_CAPTURE: Regex = Regex::new(r"!so\s@?(\w+)").unwrap();
    static ref URL_CAPTURE: Regex = Regex::new(r"(.*)-preview-").unwrap();
}

#[derive(Serialize)]
pub struct ClipData {
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


    let streamer = match get_target_streamer_id(target_channel.clone()).await {
        Some(mut res) => match res.data.len() {
            1 => res.data.pop().unwrap(),
            _ => {
                error!("zero or too many users found for username: {}", target_channel);
                return
            }
        },
        None => return
    };


    let clips =  match fetch_clips(streamer.id).await {
        Some(res) => match res.data.len() {
            0 => {
                error!("zero clips were found for {}", target_channel);
                return;
            },
            _ => res.data
        },
        None => return
    };

    info!("fetched {} clips for channel: {}", clips.len(), streamer.display_name);

    let rng = rand::thread_rng().gen_range(0..clips.len());
    let clip = clips.get(rng).expect("error extracting clip from map");
    let clip_url = format_clip_url(clip.thumbnail_url.clone()).expect("error formatting clip url");
    let clip_duration = clip.duration;

    let clip_data: ClipData = ClipData { 
        clip_url, 
        streamer: streamer.display_name, 
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