#[path = "../src/main.rs"]
mod application;

use std::sync::Arc;

use application::rocket;
use rocket::local::asynchronous::Client;

static mut CLIENT: Option<Arc<Client>> = None;

pub async fn client() -> Arc<Client> {
    if let Some(client) = unsafe { CLIENT.as_ref() } {
        return Arc::clone(client);
    } 
    unsafe {
        let client = Arc::new(Client::tracked(rocket()).await.unwrap());
        CLIENT = Some(Arc::clone(&client));

        client
    }
}
