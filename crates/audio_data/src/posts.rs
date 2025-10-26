use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::posts;

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub author_id: Uuid,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreatePost {
    pub title: String,
    pub description: String,
    pub author_id: Uuid,
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatePost {
    title: Option<String>,
    description: Option<String>,
    author_id: Option<Uuid>,
}

#[derive(Identifiable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DeletePost {
    pub id: Uuid,
}
