use diesel::{
    Queryable,
    Identifiable,
    Associations,
};

use juniper::GraphQLObject;
use chrono::prelude::*;

use super::schema::{
    articles,
    links,
};

#[derive(Queryable, GraphQLObject)]
pub struct User {
    pub email: String,
    pub username: String,
    #[graphql(skip)]
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub bio: String,
    pub avatar: String,
    pub website: String,
    pub gpg: Option<String>,
}

#[derive(Identifiable, Queryable)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub format: String,
    pub user_email: String,
    pub created_at: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, GraphQLObject)]
#[belongs_to(Article, foreign_key = "article_id")]
pub struct Link {
    pub id: i32,
    pub href: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub clicked_at: Option<NaiveDateTime>,
    pub clicks: i32,
    pub seen: i32,
    pub article_id: i32,
    pub created_at: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(Queryable, GraphQLObject)]
pub struct Comment {
    pub article_id: i32,
    pub comment_no: Option<i32>,
    pub body: String,
    pub show_email: bool,
    pub commentor_name: String,
    pub commentor_email: String,
    pub reply_on: i32,
    pub user: Option<i32>,
    pub created_at: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(Queryable, GraphQLObject)]
pub struct UserSocials {
    pub email: String,
    pub social: String,
}

#[derive(Queryable, GraphQLObject)]
pub struct ArticleTags {
    pub article_id: i32,
    pub tag: String,
}
