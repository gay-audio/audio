use diesel::{
    Selectable,
    prelude::{AsChangeset, Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::authors;

#[derive(Queryable, Selectable, Deserialize, Serialize, Debug)]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Author {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateAuthor {
    name: String,
}

#[derive(AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateAuthor {
    name: Option<String>,
}

#[derive(Identifiable)]
#[diesel(table_name = authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DeleteAuthor {
    pub id: Uuid,
}

