use diesel_async::{
    pooled_connection::{
        deadpool::Pool, AsyncDieselConnectionManager, PoolError,
    },
    AsyncPgConnection,
};

pub type Db = AsyncDieselConnectionManager<AsyncPgConnection>;
pub type DbPool = Pool<AsyncPgConnection>;