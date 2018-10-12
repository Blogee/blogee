use {
    diesel::{
        prelude::*,
        r2d2,
    },
};

use ::actix::prelude::*;

pub type DBPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub struct DbExecutor(DBPool);

impl DbExecutor {
    pub fn new(pool: DBPool) -> DbExecutor {
        DbExecutor(pool)
    }
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
