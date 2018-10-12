use juniper::{
    FieldResult,
    RootNode,
};

use crate::{
    db::models::*,
    graphql::GraphQLExecutor,
};

pub struct QueryRoot;
pub struct MutationRoot;

use diesel::prelude::*;

graphql_object!(QueryRoot: GraphQLExecutor |&self| {
    field user(&executor, email: String) -> FieldResult<User> {
        use crate::db::schema::user::dsl;

        let db = executor.context().db_pool.get()?;
        let mut result = dsl::user
            .filter(dsl::email.eq(&email))
            .load::<User>(&*db)
            .expect("cant' execute operation in db");

        match result.pop() {
            Some(item) => Ok(item),
            None => Err("Can't find User!".to_owned())? ,
        }

    }

    field article(&executor, id: i32) -> FieldResult<Article> {
        use crate::db::schema::article::dsl;


        let db = executor.context().db_pool.get()?;
        let mut result = dsl::article
            .filter(dsl::id.eq(&id))
            .load::<Article>(&*db)
            .expect("cant' execute operation in db");

        match result.pop() {
            Some(item) => Ok(item),
            None => Err("Can't find Article!".to_owned())? ,
        }
    }
});

graphql_object!(Article: GraphQLExecutor |&self| {
    field id() -> i32 {
        self.id
    }

    field title() -> &str {
        self.title.as_str()
    }

    field body() -> &str {
        self.body.as_str()
    }

    field format() -> &str {
        self.format.as_str()
    }

    field createdAt() -> Option<NaiveDateTime> {
        self.created_at
    }

    field lastModified() -> Option<NaiveDateTime> {
        self.last_modified
    }

    field userEmail() -> &str {
        self.user_email.as_str()
    }

    field links(&executor) -> FieldResult<Option<Vec<Link>>> {
        use crate::db::schema::link::dsl;
        use diesel::prelude::*;

        let db = executor.context().db_pool.get()?;
        let result = dsl::link
            .filter(dsl::article_id.eq(&self.id))
            .load::<Link>(&*db)
            .expect("cant' execute operation in db");

        match result.len() {
            0 => Ok(None),
            _ => Ok(Some(result)),
        }
    }
});

use chrono::prelude::*;

graphql_object!(MutationRoot: GraphQLExecutor |&self| {

});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
