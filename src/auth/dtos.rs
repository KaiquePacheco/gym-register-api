use super::validators::is_email;

use rocket::{
    form::FromForm,
    serde::{Deserialize, Serialize},
};
use uuid::Uuid;

#[derive(FromForm)]
pub struct SignIn<'r> {
    #[field(validate = is_email())]
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(FromForm)]
pub struct SignUp<'r> {
    pub username: &'r str,
    #[field(validate = is_email())]
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
