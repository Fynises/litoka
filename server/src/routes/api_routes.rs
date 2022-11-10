use actix_web::{web, get, Responder, Result};
use serde::Serialize;
use crate::lib;

#[allow(non_snake_case)]
#[derive(Serialize)]
struct WebSocketUrl {
    wsUrl: String,
}

#[get("/api/getwebsocket")]
pub async fn get_web_socket() -> Result<impl Responder> {
    let res = WebSocketUrl {
        wsUrl: lib::CONFIG.ws_url.to_string()
    };
    Ok(web::Json(res))
}