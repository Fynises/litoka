use regex::Regex;
use lazy_static::lazy_static;
use log::{info, error, warn};
use crate::{
    db::functions::{
        clips::get_clip,
        games::get_game_from_db,
        users,
    },
    irc_processor::command_parser::TwitchMessage,
    client_websocket::{
        shoutout_structs::{ClientConnectOptions, SoClipData},
        session_map::SESSION,
    },
};



lazy_static! {
    static ref TARGET_CHANNEL_CAPTURE: Regex = Regex::new(r"!so\s@?(\w+)").unwrap();
    static ref URL_CAPTURE: Regex = Regex::new(r"(.*)-preview-").unwrap();
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

async fn execute_shoutout(target_channel: String ,client_uuid: &String, _options: &ClientConnectOptions) {

    let streamer = match users::get_user(target_channel.clone()).await {
        Some(res) => res,
        None => return
    };

    let clip_data =  match get_clip(streamer.id, _options.to_owned()).await {
        Some(res) => res.data,
        None => return
    };

    let game = match get_game_from_db(clip_data.game_id.clone()).await {
        Some(game) => Some(game.name),
        None => {
            warn!("error getting game name for id {}, defaulting to none", clip_data.game_id);
            None
        },
    };

    let clip_url = format_clip_url(clip_data.thumbnail_url.clone()).expect("error formatting clip url");
    let clip_duration = clip_data.duration;

    let clip_data: SoClipData = SoClipData { 
        clip_url, 
        streamer: streamer.login, 
        profile_pic: streamer.profile_image_url,
        clipper: Some(clip_data.creator_name),
        game,
        clip_duration
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