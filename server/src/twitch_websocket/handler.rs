use actix_http::ws::Frame;
use actix_ws::Message;
use futures_util::{sink::SinkExt as _, stream::StreamExt as _};
use tokio::sync::mpsc;
use crate::lib::CONFIG;
use crate::irc_processor::message_parser::parse_message;
use super::twitch_ws::CONNECTION;

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

        loop {
            tokio::select! {
                res = connection.next() => {
                    match res {
                        None => return,
                        Some(body) => {
                            match body.expect("error in recieving message from twitch irc") {
                                Frame::Text(text) => parse_message(std::str::from_utf8(&text).unwrap()),
                                Frame::Binary(_) => todo!(),
                                Frame::Continuation(_) => todo!(),
                                Frame::Ping(_) => todo!(),
                                Frame::Pong(_) => todo!(),
                                Frame::Close(_) => todo!(), 
                            }
                        }
                    }
                }
                res = twitch_rx.recv() => {
                    match res {
                        None => return,
                        Some(body) => {
                            connection.send(Message::Text(body.into())).await.unwrap()
                        }
                    }
                }
            }
        }
}