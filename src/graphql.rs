pub mod executor;
pub mod objects;

pub use self::{
    objects::{
        Schema,
        create_schema,
    },
};

use {
    serde_derive::{
        Serialize,
        Deserialize,
    },
    futures::future::Future,
    juniper::http::{
        graphiql::graphiql_source,
        GraphQLRequest,
    },
    actix::{
        prelude::*,
    },
    actix_web::{
        AsyncResponder,
        Error,
        FutureResponse,
        HttpRequest,
        HttpResponse,
        Json,
        State,
    },
};

use crate::{
    blogee_server::{
        AppState,
    },
};

pub use self::executor::GraphQLExecutor;

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

pub fn graphiql(_req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    Ok(HttpResponse::Ok()
       .content_type("text/html; charset=utf-8")
       .body(html))
}

pub fn graphql(
    (st, data): (State<AppState>, Json<GraphQLData>),
) -> FutureResponse<HttpResponse> {
    st.executor
        .send(data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok()
                          .content_type("application/json")
                          .body(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
