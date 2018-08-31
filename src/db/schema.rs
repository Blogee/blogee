table! {
    article (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        format -> Text,
        created_at -> Nullable<Timestamp>,
        last_modified -> Nullable<Timestamp>,
        user_email -> Text,
    }
}

table! {
    article_tags (article_id, tag) {
        article_id -> Integer,
        tag -> Text,
    }
}

table! {
    comment (article_id, comment_no) {
        article_id -> Integer,
        comment_no -> Nullable<Integer>,
        body -> Text,
        created_at -> Nullable<Timestamp>,
        last_modified -> Nullable<Timestamp>,
        show_email -> Nullable<Bool>,
        commentor_name -> Text,
        commentor_email -> Text,
        reply_on -> Nullable<Integer>,
        user -> Nullable<Integer>,
    }
}

table! {
    link (id) {
        id -> Integer,
        href -> Text,
        title -> Text,
        description -> Text,
        clicked_at -> Nullable<Timestamp>,
        clicks -> Integer,
        seen -> Integer,
        article_id -> Integer,
    }
}

table! {
    user (email) {
        email -> Text,
        username -> Text,
        password -> Text,
        first_name -> Text,
        last_name -> Text,
        bio -> Text,
        avatar -> Text,
        website -> Text,
        gpg -> Text,
    }
}

table! {
    user_socials (email, social) {
        email -> Text,
        social -> Text,
    }
}

joinable!(article -> user (user_email));
joinable!(article_tags -> article (article_id));
joinable!(comment -> article (article_id));
joinable!(link -> article (article_id));
joinable!(user_socials -> user (email));

allow_tables_to_appear_in_same_query!(
    article,
    article_tags,
    comment,
    link,
    user,
    user_socials,
);
