#![warn(rust_2018_idioms)]

use serde_derive::{
    Serialize,
    Deserialize,
};

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate diesel;

use diesel::{Queryable, };

pub mod graphql;
pub mod db;
pub mod blogee_server;

// use diesel::{
//     prelude::*,
//     sqlite::SqliteConnection,
// };
// use dotenv::dotenv;
// use std::env;

// pub fn establish_connection() -> SqliteConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set as environment variable!");
//     SqliteConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }
