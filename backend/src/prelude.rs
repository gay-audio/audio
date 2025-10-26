use diesel::r2d2::{ConnectionManager, Pool};

pub type PoolManager<C> = Pool<ConnectionManager<C>>;

pub use crate::{
    routes::posts::{create_post, delete_post, get_post_by_id, update_post},
    types::{json::Json, ron::Ron},
};
