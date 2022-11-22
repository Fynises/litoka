use actix_web::routes;
use actix_files::NamedFile;
use std::io::Result;
use crate::lib;

#[routes]
#[get("/")]
#[get("/config/{_a}")]
#[get("/tools")]
#[get("/tools/{_a}")]
pub async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open(lib::CONFIG.index_path.as_str()).unwrap())
}