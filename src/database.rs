use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};

use super::config::Configs;

pub fn create_connection_pool(configs: &Configs) -> Pool<AsyncPgConnection> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&configs.db_url);

    Pool::builder(manager)
        .max_size(configs.db_max_conns)
        .build()
        .unwrap()
}
