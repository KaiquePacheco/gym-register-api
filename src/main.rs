use dotenvy::dotenv_override;
use rocket::{get, launch, routes, Rocket};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello world!"
}

#[launch]
fn rocket() -> _ {
    let _ = dotenv_override();

    Rocket::build().mount("/", routes![hello_world])
}
