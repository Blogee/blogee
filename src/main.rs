extern crate actix_web;
extern crate actix;
extern crate futures;
extern crate dotenv;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate diesel;

extern crate blogee;

mod models;
mod graphql;

use models::{
    create_schema,
};
use graphql::{
    graphql,
    graphiql,
    GraphQLExecutor,
    AppState,
};

use actix_web::{
    http,
    middleware,
    server,
    fs,
    App,
};

// use diesel::{
//     prelude::*,
//     sqlite::SqliteConnection,
// };

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("blogee");

    let schema = std::sync::Arc::new(create_schema());
    let addr = actix::SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone()));

    server::new(move || {
        App::with_state(AppState{executor: addr.clone()})
            .middleware(middleware::Logger::default())
            .resource("/graphql", |r| r.method(http::Method::POST).with(graphql))
            .resource("/graphiql", |r| r.method(http::Method::GET).h(graphiql))
            .handler("/", fs::StaticFiles::new("./www/")
                     .index_file("index.html")
            )
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
