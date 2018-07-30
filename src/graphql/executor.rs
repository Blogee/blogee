use {
    serde_json,
    actix::{
        prelude::*,
    },
    actix_web::{
        Error,
    },
    crate::{
        graphql::{
            GraphQLData,
        },
        models::{
            Schema,
        },
        db::{
            DBPool,
        },
    },
};

pub struct GraphQLExecutor {
    pub schema: std::sync::Arc<Schema>,
    pub db_pool: DBPool,
}

impl GraphQLExecutor {
    pub fn new(schema: std::sync::Arc<Schema>, db_pool: DBPool) -> GraphQLExecutor {
        GraphQLExecutor {
            schema,
            db_pool,
        }
    }
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &self);
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
        }
}
