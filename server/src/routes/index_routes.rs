use actix_web::routes;
use actix_files::NamedFile;
use std::io::Result;
use std::path::Path;

#[routes]
#[get("/")]
#[get("/shoutout")]
pub async fn index() -> Result<NamedFile>{
    Ok(NamedFile::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("../public/index.html"))?)
}