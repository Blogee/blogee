use juniper::{
    FieldResult,
        RootNode,
};

#[derive(Queryable, GraphQLObject)]
struct User {
    email: String,
    username: String,
    password: String,
    first_name: String,
    last_name: String,
    bio: String,
    avatar: String,
    website: String,
    gpg: String,
}

#[derive(Queryable, GraphQLObject)]
struct Article {
    id: String,
    title: String,
    body: String,
    format: String,
    created_at: String,
    last_modified: String,
    user_email: String,
}

#[derive(Queryable, GraphQLObject)]
struct Link {
    id: String,
    href: String,
    title: String,
    description: String,
    clicked_at: String,
    clicks: String,
    seen: String,
    article_id: String,
}

#[derive(Queryable, GraphQLObject)]
struct Comment {
    article_is: String,
    comment_no: String,
    body: String,
    created_at: String,
    last_modified: String,
    show_email: bool,
    commentor_name: String,
    commentor_email: String,
    reply_on: String,
    user: String,
}

#[derive(Queryable, GraphQLObject)]
struct UserSocials {
    email: String,
    social: String,
}

#[derive(Queryable, GraphQLObject)]
struct ArticleTags {
    article_id: String,
    tag: String,
}

// #[derive(GraphQLEnum)]
// enum Episode {
//     NewHope,
//     Empire,
//     Jedi,
// }

// #[derive(GraphQLObject)]
// #[graphql(description = "A humanoid creature in the Star Wars universe")]
// struct Human {
//     id: String,
//     name: String,
//     appears_in: Vec<Episode>,
//     home_planet: String,
// }

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewUser {
    email: String,
    username: String,
    password: String,
}

pub struct QueryRoot;

use crate::graphql::GraphQLExecutor;

graphql_object!(QueryRoot: GraphQLExecutor |&self| {
    field user(&executor, email: String) -> FieldResult<User> {
        use crate::db::schema::user::dsl::*;
        use diesel::prelude::*;

        let db = executor.context().db_pool.get()?;
        let mut result = user
            .filter(email.eq(&email))
            .load::<User>(&*db)
            .expect("can't find user in db");
        // Ok(User {
        //     email: "admin@admin.com".to_owned(),
        //     username: "admin".to_owned(),
        //     password: "xxx".to_owned(),
        //     first_name: "admin".to_owned(),
        //     last_name: "admin".to_owned(),
        //     bio: "Blogee admin bio.".to_owned(),
        //     avatar: "qweasdzxc".to_owned(),
        //     website: "www.blogee.com".to_owned(),
        //     gpg: "aaazaaazaaa".to_owned(),
        // })
        match result.pop() {
            Some(item) => Ok(item),
            None => Ok(User {
                email: "admin@admin.com".to_owned(),
                username: "admin".to_owned(),
                password: "xxx".to_owned(),
                first_name: "admin".to_owned(),
                last_name: "admin".to_owned(),
                bio: "Blogee admin bio.".to_owned(),
                avatar: "qweasdzxc".to_owned(),
                website: "www.blogee.com".to_owned(),
                gpg: "aaazaaazaaa".to_owned(),
            }),
        }
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: GraphQLExecutor |&self| {
    field createUser(&executor, new_user: NewUser) -> FieldResult<User> {
        Ok(User {
            email: new_user.email,
            username: new_user.username,
            password: new_user.password,
            first_name: "admin".to_owned(),
            last_name: "admin".to_owned(),
            bio: "Blogee admin bio.".to_owned(),
            avatar: "qweasdzxc".to_owned(),
            website: "www.blogee.com".to_owned(),
            gpg: "aaazaaazaaa".to_owned(),
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
