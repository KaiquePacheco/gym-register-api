use std::env;

use diesel_async::{
    pooled_connection::{deadpool::{Object, Pool}, AsyncDieselConnectionManager},
    AsyncPgConnection,
};

pub fn create_connection_pool() -> Pool<AsyncPgConnection> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

    let max_connections: usize = env::var("DATABASE_MAX_CONNECTIONS")
        .expect("DATABASE_MAX_CONNECTIONS environment variable not found")
        .parse()
        .expect("Could not parse to unsigned integer the environment variable 'DATABASE_MAX_CONNECTIONS'");

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);

    Pool::builder(manager)
        .max_size(max_connections)
        .build()
        .unwrap()
}
