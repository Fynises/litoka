use actix_web::{web, get, Responder, Result, HttpRequest, HttpResponse, Error};
use tokio::task::spawn_local;

use super::handler;

#[get("/ws/")]
pub async fn client_ws(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    spawn_local(handler::client_ws(
        session,
        msg_stream,
    ));
    Ok(res)
}