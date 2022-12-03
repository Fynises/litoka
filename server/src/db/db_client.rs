use mongodb::Client;
use once_cell::sync::OnceCell;

pub static DB_CLIENT: OnceCell<Client> = OnceCell::new();