use rocket::async_test;

use crate::application::config::Configs;

#[path = "../src/main.rs"]
mod application;

#[async_test]
async fn can_estabilish_connection_pool_with_database() {
    use application::database;

    let configs = Configs::default();
    let pool = database::create_connection_pool(&configs);

    let _ = pool
        .get()
        .await
        .expect("Couldn't estabilish connection with database");
}
