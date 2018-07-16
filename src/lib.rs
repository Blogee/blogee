extern crate actix_web;
pub extern crate actix;
extern crate futures;
extern crate dotenv;
pub extern crate env_logger;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate diesel;

mod graphql;
mod models;

use diesel::{
    prelude::*,
    sqlite::SqliteConnection,
};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set as environment variable!");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
