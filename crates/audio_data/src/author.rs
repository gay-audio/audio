use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::authors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Author {
    pub id: Uuid,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::authors)]
pub struct CreateAuthor {
    pub name: String,
}
