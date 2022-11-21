use actix_web::{App, HttpServer, middleware::Logger};
use tokio::task::spawn_local;
use std::io::Result;
use actix_files::Files;

mod routes;
mod lib;
mod client_websocket;
mod twitch_websocket;
mod irc_processor;
mod twitch_api;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    if !lib::CONFIG.client_only {
        spawn_local(twitch_websocket::handler::init_socket());
    }

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(routes::index_routes::index)
            .service(routes::api_routes::get_web_socket)
            .service(client_websocket::websocket_route::client_ws)
            .service(Files::new("/dist", lib::CONFIG.dist_path.as_str())
                .show_files_listing().use_last_modified(true)
            )
    })
    .bind(("127.0.0.1", lib::CONFIG.port))?
    .workers(lib::CONFIG.workers.into())
    .run()
    .await
}
