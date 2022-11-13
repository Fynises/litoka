use std::time::{Duration, Instant};

use actix_ws::Message;
use futures_util::{
    future::{select, Either},
    StreamExt as _,
};
use tokio::{pin, time::interval};

use crate::client_websocket::shoutout_structs::{ClientConnectMessage, ClipData};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(20);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

pub async fn client_ws(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
) {
    println!("connected");

    let mut name: Option<String> = None;
    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let close_reason = loop {
        let tick = interval.tick();
        pin!(tick);

        match select(msg_stream.next(), tick).await {
            Either::Left((Some(Ok(msg)), _)) => {
                println!("msg: {msg:?}");

                match msg {
                    Message::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        session.pong(&bytes).await.unwrap();
                    }

                    Message::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    Message::Text(text) => {
                        let client_connect_message: ClientConnectMessage = serde_json::from_str(&text).expect("json error");
                        //debug
                        println!("message recieved: {client_connect_message:?}");
                        //debug return with message
                        let test_clipdata: ClipData = ClipData { 
                            clip_url: "test".to_string(), 
                            streamer: "test".to_string(), 
                            profile_pic: "test".to_string(), 
                            clip_duration: 50 
                        };
                        session.text(serde_json::to_string(&test_clipdata).expect("json error")).await.unwrap();
                    },

                    Message::Binary(_bin) => {
                        println!("unexpected binary message");
                    }

                    Message::Close(reason) => break reason,

                    _ => {
                        break None;
                    }
                }
            }

            Either::Left((Some(Err(err)), _)) => {
                println!("{}", err);
            }

            Either::Left((None, _)) => break None,

            Either::Right((_inst, _)) => {
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    println!("client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting");
                    break None;
                }

                let _ = session.ping(b"").await;
            }
        };
    };

    let _ = session.close(close_reason).await;
}