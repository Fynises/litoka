use actix_web::{web, App, HttpServer, middleware::Logger};
use std::io::Result;
use actix_files::Files;
use tokio::{
    task::{spawn},
    try_join,
};

mod routes;
mod lib;
mod client_websocket;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let (ws_server, server_tx) = client_websocket::websocket_server::WsServer::new();

    let ws_server = spawn(ws_server.run());
    
    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(server_tx.clone()))
            .service(routes::index_routes::index)
            .service(routes::api_routes::get_web_socket)
            .service(client_websocket::websocket_route::client_ws)
            .service(Files::new("/dist", lib::CONFIG.dist_path.as_str())
                .show_files_listing().use_last_modified(true)
            )
    })
    .bind(("127.0.0.1", lib::CONFIG.port))?
    .workers(lib::CONFIG.workers.into())
    .run();

    try_join!(http_server, async move { ws_server.await.unwrap()})?;
    Ok(())
}
