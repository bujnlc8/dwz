pub mod controllers;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
