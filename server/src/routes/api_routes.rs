use actix_web::{web, get, Responder, Result};
use serde::Serialize;
use crate::lib;

#[derive(Serialize)]
struct WebSocketUrl {
    ws_url: String,
}

#[derive(Serialize)]
struct TwitchAuthParams {
    client_id: String,
    redirect_uri: String,
    scope: String,
}

#[get("/api/getwebsocket")]
pub async fn get_web_socket() -> Result<impl Responder> {
    let res = WebSocketUrl {
        ws_url: lib::CONFIG.ws_url.to_string()
    };
    Ok(web::Json(res))
}

#[get("/api/get-auth-params")]
pub async fn get_auth_params() -> Result<impl Responder> {
    let res: TwitchAuthParams = TwitchAuthParams {
        client_id: lib::CONFIG.twitch_client_id.clone(),
        redirect_uri: lib::CONFIG.twitch_auth_redirect.clone(),
        scope: "chat:read chat:edit".to_string(), //hardcoded for now
    };
    Ok(web::Json(res))
}