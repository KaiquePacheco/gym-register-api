use rocket::{
    form::FromForm,
    serde::{Deserialize, Serialize},
};
use uuid::Uuid;

#[derive(FromForm)]
pub struct SignIn<'r> {
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(FromForm)]
pub struct SignUp<'r> {
    pub username: &'r str,
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TokenContent {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
}
