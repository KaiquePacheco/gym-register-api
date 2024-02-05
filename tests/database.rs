use dotenvy::dotenv_override;
use rocket::async_test;

#[path = "../src/main.rs"]
mod application;

#[async_test]
async fn can_estabilish_connection_pool_with_database() {
    use application::database;

    let _ = dotenv_override();
    let pool = database::create_connection_pool();

    let _ = pool
        .get()
        .await
        .expect("Couldn't estabilish connection with database");
}
