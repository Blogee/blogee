table! {
    articles (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        format -> Text,
        user_email -> Text,
        created_at -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    articles_tags (article_id, tag) {
        article_id -> Integer,
        tag -> Text,
    }
}

table! {
    comments (article_id, comment_no) {
        article_id -> Integer,
        comment_no -> Nullable<Integer>,
        body -> Text,
        show_email -> Nullable<Bool>,
        commentor_name -> Text,
        commentor_email -> Text,
        reply_on -> Nullable<Integer>,
        user -> Nullable<Integer>,
        created_at -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    links (id) {
        id -> Integer,
        href -> Text,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        clicked_at -> Nullable<Timestamp>,
        clicks -> Integer,
        seen -> Integer,
        article_id -> Integer,
        created_at -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    users (email) {
        email -> Text,
        username -> Text,
        password -> Text,
        first_name -> Text,
        last_name -> Text,
        bio -> Text,
        avatar -> Text,
        website -> Text,
        gpg -> Nullable<Text>,
    }
}

table! {
    users_socials (user_email, social) {
        user_email -> Text,
        social -> Text,
    }
}

joinable!(articles -> users (user_email));
joinable!(articles_tags -> articles (article_id));
joinable!(comments -> articles (article_id));
joinable!(links -> articles (article_id));
joinable!(users_socials -> users (user_email));

allow_tables_to_appear_in_same_query!(
    articles,
    articles_tags,
    comments,
    links,
    users,
    users_socials,
);
