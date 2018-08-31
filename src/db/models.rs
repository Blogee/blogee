use diesel::{
    Queryable,
};

use chrono::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub email: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub bio: String,
    pub avatar: String,
    pub website: String,
    pub gpg: String,
}

#[derive(Queryable)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub format: String,
    pub created_at: Option<NaiveDateTime>,
    pub last_modified: Option<NaiveDateTime>,
    pub user_email: String,
}

#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    pub href: String,
    pub title: String,
    pub description: String,
    pub clicked_at: Option<NaiveDateTime>,
    pub clicks: i32,
    pub seen: i32,
    pub article_id: i32,
}

#[derive(Queryable)]
pub struct Comment {
    pub article_is: String,
    pub comment_no: String,
    pub body: String,
    pub created_at: String,
    pub last_modified: String,
    pub show_email: bool,
    pub commentor_name: String,
    pub commentor_email: String,
    pub reply_on: String,
    pub user: String,
}

#[derive(Queryable)]
pub struct UserSocials {
    pub email: String,
    pub social: String,
}

#[derive(Queryable)]
pub struct ArticleTags {
    pub article_id: String,
    pub tag: String,
}
