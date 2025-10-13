use diesel::r2d2::{ConnectionManager, Pool};

pub type PoolManager<C> = Pool<ConnectionManager<C>>;

pub use crate::types::{Serde, ron::Ron};
