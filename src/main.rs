#![warn(rust_2018_idioms)]

use {
    actix_web::{
        http,
        middleware,
        server,
        fs,
        App
    },
    diesel::{
        r2d2,
        sqlite::SqliteConnection,
    },
    dotenv::{
        dotenv,
    },
    blogee::{
        graphql::{
            graphql,
            graphiql,
            GraphQLExecutor,
            create_schema,
        },
        db::{
            DbExecutor,
        },
        blogee_server::{
            AppState,
        },
    },
};

fn main() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let cpus = num_cpus::get();
    let database_url = std::env::var("DATABASE_URL").expect("can't read DB env url.");
    let sys = actix::System::new("blogee");

    let schema = std::sync::Arc::new(create_schema());
    let db_manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    let db_pool = r2d2::Pool::builder()
        .max_size(cpus as u32)
        .build(db_manager)
        .expect("Pool connection error");
    let db_executor_pool = db_pool.clone();

    let graphql_addr = actix::SyncArbiter::start(
        cpus,
        move || GraphQLExecutor::new(schema.clone(), db_pool.clone()));
    let db_pool_addr = actix::SyncArbiter::start(
        cpus,
        move || DbExecutor::new(db_executor_pool.clone()));

    server::new(move || {
        App::with_state(AppState{
            executor: graphql_addr.clone(),
            db_pool: db_pool_addr.clone(),
        })
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
