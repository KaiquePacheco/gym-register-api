pub mod config;
pub mod database;
pub mod schema;

pub mod auth;
pub mod users;

use config::Configs;
use database::ConnectionPool;
use rocket::{launch, routes, Rocket};

#[launch]
fn rocket() -> _ {
    let configs = Configs::default();
    let pool: ConnectionPool = database::create_connection_pool(&configs);

    Rocket::build()
        .manage(pool)
        .manage(configs)
        .mount("/auth", routes![auth::sign_in, auth::sign_up])
}
