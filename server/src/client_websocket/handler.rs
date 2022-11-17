use std::{time::{Duration, Instant}};
use actix_ws::Message;
use futures_util::{
    future::{select, Either},
    StreamExt as _,
};
use uuid::Uuid;
use tokio::{pin, time::interval, sync::mpsc};
use serde_json;
use crate::client_websocket::shoutout_structs::ClientConnectMessage;
use super::session_map::SESSION;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(20);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

pub async fn client_ws(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
) {
    println!("connected");

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();

    let uuid: String = Uuid::new_v4().to_string();
    let mut channel: String = String::new();
    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let close_reason = loop {
        let tick = interval.tick();
        pin!(tick);

        let msg_rx = conn_rx.recv();
        pin!(msg_rx);

        let messages = select(msg_stream.next(), msg_rx);

        match select(messages, tick).await {
            Either::Left((Either::Left((Some(Ok(msg)), _)), _)) => {

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
                        channel = client_connect_message.options.channel.clone();
                        SESSION.lock().unwrap().add(conn_tx.clone(), client_connect_message.options, uuid.clone());
                    },

                    Message::Binary(_bin) => {
                        println!("unexpected binary message");
                    }

                    Message::Close(reason) => {
                        println!("{reason:?}");
                        SESSION.lock().unwrap().close(uuid.clone(), channel.clone());
                    },

                    _ => {
                        break None;
                    }
                }
            }

            // client websocket stream error
            Either::Left((Either::Left((Some(Err(err)), _)), _)) => {
                println!("{}", err);
                break None;
            }

            // client websocket stream ended
            Either::Left((Either::Left((None, _)), _)) => break None,

            Either::Left((Either::Right((Some(server_message), _)), _)) => {
                session.text(server_message).await.unwrap();
            }

            // all connection's message senders were dropped
            Either::Left((Either::Right((None, _)), _)) => unreachable!(
                "all connection message senders were dropped; server may have panicked"
            ),

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