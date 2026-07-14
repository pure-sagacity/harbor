use diesel::prelude::*;
use dotenvy::dotenv;

pub mod models;
pub mod schema;

pub fn establish_connection(url: String) -> SqliteConnection {
    dotenv().ok();

    let resolved_path = std::fs::canonicalize(&url)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or(url);

    SqliteConnection::establish(&resolved_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &resolved_path))
}
