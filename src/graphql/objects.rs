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

graphql_object!(Link: () |&self| {

    field id() -> i32 {
        self.id
    }

    field href() -> &str {
        &self.href
    }

    field title() -> &str {
        &self.title
    }

    field description() -> &str {
        &self.description
    }

    field clickedAt() -> Option<NaiveDateTime> {
        self.clicked_at
    }

    field clicks() -> i32 {
        self.clicks
    }

    field seen() -> i32 {
        self.seen
    }

    field articleId() -> i32 {
        self.article_id
    }
});

graphql_object!(Comment: () |&self| {
    field articleIs() -> &str {
        &self.article_is
    }

    field commentNo() -> &str {
        &self.comment_no
    }

    field body() -> &str {
        &self.body
    }

    field createdAt() -> &str {
        &self.created_at
    }

    field lastModified() -> &str {
        &self.last_modified
    }

    field showEmail() -> &bool {
        &self.show_email
    }

    field commentorName() -> &str {
        &self.commentor_name
    }

    field commentorEmail() -> &str {
        &self.commentor_email
    }

    field replyOn() -> &str {
        &self.reply_on
    }

    field user() -> &str {
        &self.user
    }
});

graphql_object!(UserSocials: () |&self| {
    field email() -> &str {
        &self.email
    }

    field social() -> &str {
        &self.social
    }
});

graphql_object!(ArticleTags: () |&self| {
    field articleId() -> &str {
        &self.article_id
    }

    field tag() -> &str {
        &self.tag
    }
});

graphql_object!(User: () |&self| {
    field email() -> &str {
        &self.email
    }

    field userName() -> &str {
        &self.username
    }

    field firstName() -> &str {
        &self.first_name
    }

    field lastName() -> &str {
        &self.last_name
    }

    field bio() -> &str {
        &self.bio
    }

    field avatar() -> &str {
        &self.avatar
    }

    field website() -> &str {
        &self.website
    }

    field gpg() -> &str {
        &self.gpg
    }
});


graphql_object!(MutationRoot: GraphQLExecutor |&self| {

});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
