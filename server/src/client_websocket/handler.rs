use std::time::{Duration, Instant};

use actix_ws::Message;
use futures_util::{
    future::{select, Either},
    StreamExt as _,
};
use tokio::{pin, sync::mpsc, time::interval};

use crate::client_websocket::shoutout_structs::ClientConnectMessage;

use super::websocket_server::{WsServerHandle, ConnId};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(20);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

pub async fn client_ws(
    ws_server: WsServerHandle,
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
) {
    println!("connected");

    let mut name: Option<String> = None;
    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let (conn_tx, mut conn_rx) = mpsc::unbounded_channel();

    let conn_id = ws_server.connect(conn_tx).await;

    let close_reason = loop {
        let tick = interval.tick();
        pin!(tick);

        let msg_rx = conn_rx.recv();
        pin!(msg_rx);

        let messages = select(msg_stream.next(), msg_rx);
        pin!(messages);

        match select(messages, tick).await {
            Either::Left((Either::Left((Some(Ok(msg)), _)), _)) => {
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

            Either::Left((Either::Left((Some(Err(err)), _)), _)) => {
                println!("{}", err);
                break None;
            }

            Either::Left((Either::Left((None, _)), _)) => break None,

            Either::Left((Either::Right((Some(chat_msg), _)), _)) => todo!(),

            Either::Left((Either::Right((None, _)), _)) => unreachable!(
                "all connections were dropped; server may have panicked"
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

    ws_server.disconnect(conn_id);

    let _ = session.close(close_reason).await;
}