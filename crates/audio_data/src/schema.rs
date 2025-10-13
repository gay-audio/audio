// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Uuid,
        #[max_length = 256]
        name -> Varchar,
    }
}

diesel::table! {
    content (id) {
        id -> Uuid,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 256]
        content_type -> Varchar,
    }
}

diesel::table! {
    content_tags_reference (id) {
        id -> Uuid,
        #[max_length = 256]
        tag_name -> Varchar,
        content_id -> Uuid,
    }
}

diesel::table! {
    tags (name) {
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 256]
        display_name -> Varchar,
    }
}

diesel::joinable!(content_tags_reference -> authors (content_id));
diesel::joinable!(content_tags_reference -> tags (tag_name));

diesel::allow_tables_to_appear_in_same_query!(authors, content, content_tags_reference, tags,);
