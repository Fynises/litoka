use actix_web::{App, HttpServer, middleware::Logger};
use std::path::Path;
use std::io::Result;
use actix_files::Files;

mod routes;
mod lib;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(routes::index_routes::index)
            .service(routes::api_routes::get_web_socket)
            .service(Files::new("/dist", Path::new(env!("CARGO_MANIFEST_DIR")).join("../dist"))
                .show_files_listing().use_last_modified(true)
            )
    })
    .bind(("127.0.0.1", lib::CONFIG.port))?
    .run()
    .await
}
