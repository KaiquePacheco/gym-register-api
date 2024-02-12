pub mod config;
pub mod database;
pub mod schema;

pub mod auth;
pub mod users;

use std::sync::Arc;

use config::Configs;
use rocket::{launch, routes, Rocket};
use users::repository::UsersRepository;

#[launch]
pub fn rocket() -> _ {
    let configs = Configs::default();

    let pool = Arc::new(database::create_connection_pool(&configs));

    let user_repository = UsersRepository(Arc::clone(&pool));

    Rocket::build()
        .manage(pool)
        .manage(configs)
        // Repositories
        .manage(user_repository)
        // Routes
        .mount("/auth", routes![auth::sign_in, auth::sign_up])
}
