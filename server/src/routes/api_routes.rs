use actix_web::{web, get, Responder, Result};
use serde::Serialize;
use crate::lib;

#[derive(Serialize)]
struct WebSocketUrl {
    ws_url: String,
}

#[get("/api/getwebsocket")]
pub async fn get_web_socket() -> Result<impl Responder> {
    let res = WebSocketUrl {
        ws_url: lib::CONFIG.ws_url.to_string()
    };
    Ok(web::Json(res))
}