use super::twitch_ws::CONNECTION;
use crate::irc_processor::message_parser::parse_message;
use crate::lib::CONFIG;
use actix_http::ws::{Frame, Item};
use actix_ws::Message;
use bytes::{Bytes, BytesMut};
use futures_util::{sink::SinkExt as _, stream::StreamExt as _};
use log::{info, warn, error};
use tokio::sync::mpsc;

struct ContinuationBuffer {
    buffer: BytesMut,
}

impl ContinuationBuffer {
    fn new() -> ContinuationBuffer {
        ContinuationBuffer {
            buffer: BytesMut::new(),
        }
    }

    async fn add_first(&mut self, bytes: Bytes) {
        self.buffer.extend_from_slice(&bytes);
    }

    async fn add_continue(&mut self, bytes: Bytes) {
        self.buffer.extend_from_slice(&bytes);
    }

    async fn add_last(&mut self, bytes: Bytes) {
        self.buffer.extend_from_slice(&bytes);
    }

    async fn get_complete(&mut self) -> Bytes {
        let output_bytes = self.buffer.to_owned().freeze();
        self.buffer.clear();
        return output_bytes;
    }
}

pub async fn init_socket() {
    let (_resp, mut connection) = awc::Client::new()
        .ws(CONFIG.twitch_chat_url.as_str())
        .connect()
        .await
        .unwrap();

    let (twitch_tx, mut twitch_rx) = mpsc::unbounded_channel();

    CONNECTION.lock().unwrap().set_sender(twitch_tx.clone()).await;

    connection.send(Message::Text("CAP REQ :twitch.tv/membership twitch.tv/tags twitch.tv/commands".into())).await.unwrap();
    connection.send(Message::Text(format!("PASS oauth:{}", CONFIG.twitch_access_token).into())).await.unwrap();
    connection.send(Message::Text("NICK opensobot".into())).await.unwrap();

    let mut continuation_buffer: ContinuationBuffer = ContinuationBuffer::new();

    loop {
        tokio::select! {
            res = connection.next() => {
                match res {
                None => (),
                    Some(body) => {
                        match body.expect("error in recieving message from twitch irc") {
                            Frame::Text(text) => {
                                // logging messages for debugging
                                let message = std::str::from_utf8(&text).unwrap();
                                info!("recieved message: {}", message);
                                parse_message(message).await;
                            }
                            Frame::Binary(_) => error!("binary is not implemented for this context"),
                            Frame::Continuation(chunk) => {
                                warn!("recieved continuation");
                                match chunk {
                                    Item::FirstText(bytes) => continuation_buffer.add_first(bytes).await,
                                    Item::FirstBinary(_) => error!("continuation is not implemented to take binaries"),
                                    Item::Continue(bytes) => continuation_buffer.add_continue(bytes).await,
                                    Item::Last(bytes) => {
                                        continuation_buffer.add_last(bytes).await;
                                        let full_msg = continuation_buffer.get_complete().await;
                                        match full_msg.len() {
                                            0 => {
                                                error!("continuation buffer was empty");
                                                continue;
                                            },
                                            _ => {
                                                // logging messages for debugging
                                                let message = std::str::from_utf8(&full_msg).unwrap();
                                                warn!("continuation message: {}", &message);
                                                parse_message(message).await;
                                            }
                                        }
                                    },
                                }
                            },
                            Frame::Ping(_) => error!("ping is not implemented for this context"),
                            Frame::Pong(_) => error!("pong is not implemented for this context"),
                            Frame::Close(_) => error!("close is not implemented for this context"),
                        }
                    }
                }
            }
            res = twitch_rx.recv() => {
                match res {
                    None => (),
                    Some(body) => {
                        connection.send(Message::Text(body.into())).await.unwrap()
                    }
                }
            }
        }
    }
}
