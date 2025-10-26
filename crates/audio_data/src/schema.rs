// @generated automatically by Diesel CLI.

diesel::table! {
    author_projects_relationship (author_id, project_id) {
        author_id -> Uuid,
        project_id -> Uuid,
    }
}

diesel::table! {
    authors (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    content (id) {
        id -> Uuid,
        title -> Text,
        content_type -> Text,
        project_id -> Uuid,
    }
}

diesel::table! {
    content_mime_types (content_type, mime_type) {
        content_type -> Text,
        mime_type -> Text,
    }
}

diesel::table! {
    content_tag_relationship (content_id, tag_id) {
        content_id -> Uuid,
        tag_id -> Uuid,
    }
}

diesel::table! {
    content_types (content_type) {
        content_type -> Text,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        author_id -> Uuid,
    }
}

diesel::table! {
    projects (id) {
        id -> Uuid,
        title -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Uuid,
        display -> Text,
    }
}

diesel::joinable!(author_projects_relationship -> authors (author_id));
diesel::joinable!(author_projects_relationship -> projects (project_id));
diesel::joinable!(content -> content_types (content_type));
diesel::joinable!(content -> projects (project_id));
diesel::joinable!(content_mime_types -> content_types (content_type));
diesel::joinable!(content_tag_relationship -> content (content_id));
diesel::joinable!(content_tag_relationship -> tags (tag_id));
diesel::joinable!(posts -> authors (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    author_projects_relationship,
    authors,
    content,
    content_mime_types,
    content_tag_relationship,
    content_types,
    posts,
    projects,
    tags,
);
