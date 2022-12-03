use crate::lib::CONFIG;
use lazy_static::lazy_static;
use mongodb::Client;
use once_cell::sync::{Lazy, OnceCell};
use tokio::runtime::Runtime;

pub static DB_CLIENT: OnceCell<Client> = OnceCell::new();