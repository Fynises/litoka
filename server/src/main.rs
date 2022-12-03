use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};
use std::io::Result;
use tokio::task::spawn_local;
use mongodb::Client;

mod client_websocket;
mod db;
mod irc_processor;
mod lib;
mod routes;
mod twitch_api;
mod twitch_websocket;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    if !lib::CONFIG.client_only {
        spawn_local(twitch_websocket::handler::init_socket());
    }

    let client = Client::with_uri_str(lib::CONFIG.mongodb_uri.clone()).await.expect("error connecting to db");
    
    db::db_client::DB_CLIENT.set(client.clone()).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(routes::index_routes::index)
            .service(routes::api_routes::get_web_socket)
            .service(client_websocket::websocket_route::client_ws)
            .service(
                Files::new("/dist", lib::CONFIG.dist_path.as_str())
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind(("127.0.0.1", lib::CONFIG.port))?
    .workers(lib::CONFIG.workers.into())
    .run()
    .await
}
