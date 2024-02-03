use rocket::{get, launch, routes, Rocket};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello world!"
}

#[launch]
fn rocket() -> _ {
    Rocket::build().mount("/", routes![hello_world])
}