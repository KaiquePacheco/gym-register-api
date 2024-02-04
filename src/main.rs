pub mod database;
pub mod schema;

use dotenvy::dotenv_override;
use database::ConnectionPool;
use rocket::{data, get, launch, routes, Rocket};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello world!"
}

#[launch]
fn rocket() -> _ {
    let _ = dotenv_override();

    let pool: ConnectionPool = database::create_connection_pool();

    Rocket::build()
        .attach(pool)
        .mount("/", routes![hello_world])
}
