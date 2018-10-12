use ::actix::prelude::*;

use crate::{
    graphql::{
        GraphQLExecutor,
    },
    db::{
        DbExecutor,
    },
};

pub struct AppState {
    pub executor: Addr<Syn, GraphQLExecutor>,
    pub db_pool: Addr<Syn, DbExecutor>,
}
