pub mod database;
pub mod schema;

use database::ConnectionPool;
use dotenvy::dotenv_override;
use rocket::{launch, routes, Rocket};

#[launch]
fn rocket() -> _ {
    let _ = dotenv_override();

    let pool: ConnectionPool = database::create_connection_pool();

    Rocket::build().manage(pool)
}
